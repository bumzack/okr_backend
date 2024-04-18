using System.Collections.Concurrent;
using BackendThingi.Api.Features.Public.Articles.Import;
using BackendThingi.Api.Model;
using BackendThingi.Api.Utils;
using Microsoft.Extensions.Options;

namespace BackendThingi.Api.Services;

public class ArticleService(ILogger<ArticleService> logger, IOptions<BackendThingiOptions> options)
{
    private readonly BackendThingiOptions _options = options.Value;

    public Task<Response> Import(bool returnItems, CancellationToken ct)
    {
        var files = GetImportFilePaths();

        var linesProcessed = 0;
        var dbRowsWritten = 0;

        ArticleLine? previousArticle = null;
        var articleWithSameCodeAndPos = new List<ArticleLine?>();
        var writeToDbArticles = new List<ArticleLine?>();
        
        foreach (var file in files)
        {
            logger.LogInformation("Importing articles from file {FilePath}", file);
            
            using var reader = new FixedWidthStreamReader<ArticleLine>(file);
            
            while (!reader.EndOfStream)
            {
                var article = reader.ReadLine();
                linesProcessed++;
                
                if (article == null)
                {
                    break;
                }
                
                if (previousArticle == null)
                {
                    articleWithSameCodeAndPos.Add(article);
                }
                else
                {
                    if (article.Code.Equals(previousArticle.Code) && article.Pos.Equals(previousArticle.Pos))
                    {
                        articleWithSameCodeAndPos.Add(article);
                    }
                    else
                    {
                        var cheapest = articleWithSameCodeAndPos.MinBy(a => a!.Price);
                        
                        if (returnItems)
                        { 
                            writeToDbArticles.Add(cheapest); 
                        }

                        dbRowsWritten++;
                    
                        articleWithSameCodeAndPos.Clear();
                        articleWithSameCodeAndPos.Add(article);
                    }
                }
                
                previousArticle = article; 
            }
            
            var last = articleWithSameCodeAndPos.MinBy(a => a!.Price);
            
            if (returnItems)
            { 
                writeToDbArticles.Add(last); 
            }

            dbRowsWritten++;
        }

        return Task.FromResult(new Response {
            LinesProcessed = linesProcessed,
            DbRowsWritten = dbRowsWritten,
            Articles = writeToDbArticles.OrderBy(a => a.Code).ThenBy(a => a.Pos).ToList()
        });
    }

    /// <summary>
    /// Retrieves the paths of import files for articles.
    /// </summary>
    /// <returns>An enumerable collection of file paths.</returns>
    /// <exception cref="Exception">Thrown when ArticlesFilesDirPath is not configured.</exception>
    private IEnumerable<string> GetImportFilePaths()
    {
        if (_options.ArticlesFilesDirPath is null)
        {
            throw new Exception("ArticlesFilesDirPath is not configured");
        }
        
        logger.LogInformation("Importing articles from files under {ArticlesFilesDirPath}", _options.ArticlesFilesDirPath);

        return Directory.GetFiles(_options.ArticlesFilesDirPath, "*.txt");
    }

    
    public Task<Response> ImportMutli(bool returnItems, CancellationToken ct)
    {
        var responses = new ConcurrentBag<Response>();
        
        var filePaths = GetImportFilePaths();
        
        Parallel.ForEach(filePaths, (filePath) =>
        {
            logger.LogInformation("Importing articles from file {FilePath}", filePath);
            responses.Add(ImportFile(filePath, returnItems));
        });

        var combinedResponse = new Response
        {
            LinesProcessed = 0,
            DbRowsWritten = 0,
            Articles = []
        };
        
        foreach (var response in responses)
        {
            combinedResponse.LinesProcessed += response.LinesProcessed;
            combinedResponse.DbRowsWritten += response.DbRowsWritten;
            combinedResponse.Articles.AddRange(response.Articles);
        }

        combinedResponse.Articles = combinedResponse.Articles.OrderBy(a => a.Code).ThenBy(a => a.Pos).ToList();
        
        return Task.FromResult(combinedResponse);
    }

    private static Response ImportFile(string filePath, bool returnItems = false)
    {
        var linesProcessed = 0;
        var dbRowsWritten = 0;
        
        ArticleLine? previousArticle = null;
        var articleWithSameCodeAndPos = new List<ArticleLine?>();
        var writeToDbArticles = new List<ArticleLine>();
        
        using (var fileStream = new FileStream(filePath, FileMode.Open, FileAccess.Read))
        using (var stream = new FixedWidthStreamReader<ArticleLine>(fileStream))
        {
            while (!stream.EndOfStream)
            {
                var article = stream.ReadLine();
                linesProcessed++;
                
                if (article != null)
                {
                    if (previousArticle == null)
                    {
                        articleWithSameCodeAndPos.Add(article);
                    }
                    else
                    {
                        if (article.Code.Equals(previousArticle.Code) && article.Pos.Equals(previousArticle.Pos))
                        {
                            articleWithSameCodeAndPos.Add(article);
                        }
                        else
                        {
                            var cheapest = articleWithSameCodeAndPos.MinBy(a => a!.Price);
                        
                            if (returnItems)
                            { 
                                writeToDbArticles.Add(cheapest); 
                            }

                            dbRowsWritten++;
                    
                            articleWithSameCodeAndPos.Clear();
                            articleWithSameCodeAndPos.Add(article);
                        }
                    }
                    
                    previousArticle = article; 
                }
            }
            
            var last = articleWithSameCodeAndPos.MinBy(a => a!.Price);
            
            if (returnItems)
            { 
                writeToDbArticles.Add(last); 
            }

            dbRowsWritten++;
        }
        
        return new Response {
            LinesProcessed = linesProcessed,
            DbRowsWritten = dbRowsWritten,
            Articles = writeToDbArticles
        };
    }
}