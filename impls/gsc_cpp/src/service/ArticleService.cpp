
#include <iostream>
#include <numeric>
#include <sstream>
#include "ArticleService.hpp"
#include "dto/ArticlelDto.hpp"
#include <numeric>
#include <fstream>
#include <filesystem>
#include <time.h>

using namespace std;
namespace fs = std::filesystem;

//oatpp::Object<PageDto<oatpp::Object<ArticleDto>>>
oatpp::data::mapping::type::VectorObjectWrapper<oatpp::data::mapping::type::DTOWrapper<ArticleDto>, oatpp::data::mapping::type::__class::Vector<oatpp::data::mapping::type::DTOWrapper<ArticleDto>>>
ArticleService::getAllArticles(const oatpp::UInt32 &offset, const oatpp::UInt32 &limit) {
    oatpp::UInt32 countToFetch = limit;

    if (limit > 10) {
        countToFetch = 10;
    }

    auto dbResult = m_database->getAllArticles(offset, countToFetch);
    OATPP_ASSERT_HTTP(dbResult->isSuccess(), Status::CODE_500, dbResult->getErrorMessage());

    auto items = dbResult->fetch<oatpp::Vector<oatpp::Object<ArticleDto>>>();
//    auto page = PageDto<oatpp::Object<ArticleDto>>::createShared();
//    page->offset = offset;
//    page->limit = countToFetch;
//    page->count = items->size();
//    page->items = items;

    return items;
}


oatpp::Object<ImportResultDto> ArticleService::importArticles() {

    string path("/home/bumzack/stoff/rust/okr_backend/rust/");

    vector<string> txt_files;

    for (const auto &entry: fs::directory_iterator(path)) {

        // Converting the path to const char * in the
        // subsequent lines
        std::filesystem::path outfilename = entry.path();
        std::string outfilename_str = outfilename.string();

        std::transform(outfilename_str.begin(), outfilename_str.end(), outfilename_str.begin(),
                       [](unsigned char c) { return std::tolower(c); });

        unsigned long i = outfilename_str.find("articles");
        cout << "outfilename_str " << outfilename_str << "find()  " << i << endl;
        if (i > 1 && i < 10000) {
            txt_files.push_back(outfilename_str);
        }
    }

    for (vector<string>::iterator t = txt_files.begin(); t != txt_files.end(); ++t) {
        cout << "txt file " << *t << endl;

        string line;
        ifstream file(*t);
        int i = 0;
        if (file.is_open()) {
            while (getline(file, line)) {
                // cout << line << "\n";
                i++;
                Article article = convert_to_article(line);

//                cout << "code " << article.code << ", articld.title " << article.title << " ,  article.cat   "
//                     << article.categories << "  ,   article.attr  " << article.attributes << "   , pos:  " << article.pos
//                     << " , price:   " << article.price << endl;

            }
            file.close();
        }
    }

    auto res = oatpp::Object<ImportResultDto>::createShared();
    res.get()->db_rows_written = 2;
    res.get()->lines_processed = 4;

    return res;
}

oatpp::Object<SysinfoDto> ArticleService::sysinfo() {
    auto res = oatpp::Object<SysinfoDto>::createShared();
    res.get()->author = "gsc";
    res.get()->language = "cpp";
    res.get()->framework = "oatpp";
    res.get()->multithreaded = false;
    res.get()->comment = "oh boy ...";

    return res;
}

Article ArticleService::convert_to_article(string line) {
    Article article;

    int start_title = LEN_CODE;
    int start_desc = start_title + LEN_TITLE;
    int start_attr = start_desc + LEN_DESC;
    int start_cat = start_attr + LEN_ATTRIBUTES;
    int start_pos = start_cat + LEN_CATEGORIES;
    int start_price = start_pos + LEN_POS;
    int start_start_date = start_price + LEN_PRICE;
    int start_end_date = start_start_date + LEN_START_DATE;
    int end_end_date = start_end_date + LEN_END_DATE;

    // cout << "start_start_date  " << start_start_date << "     start_end_date  " << start_end_date << endl;
    article.code = line.substr(0, LEN_CODE);
    article.title = line.substr(start_title, LEN_TITLE);
    article.description = line.substr(start_desc, LEN_DESC);
    article.attributes = line.substr(start_attr, LEN_ATTRIBUTES);
    article.categories = line.substr(start_cat, LEN_CATEGORIES);
    article.pos = line.substr(start_pos, LEN_POS);
    article.price = std::stod(line.substr(start_price, LEN_PRICE));
    // article.start_date= line.substr(start_start_date, LEN_START_DATE);
    // article.end_date= line.substr(start_end_date, LEN_END_DATE);

    article.code.erase(0, article.code.find_first_not_of('0'));
    article.pos.erase(0, article.pos.find_first_not_of('0'));

    rtrim(article.title);
    rtrim(article.description);
    rtrim(article.attributes);
    rtrim(article.categories);

    auto s = line.substr(start_start_date, LEN_START_DATE);
    //  cout << "s   " << s << endl;
    const long timestamp = std::stol(line.substr(start_start_date, LEN_START_DATE));
    auto ti = ctime(&timestamp);

    // cout << "start date " << ti << "   timestamp as long    " << timestamp << endl;
    return article;
}

// trim from end (in place)
inline void ArticleService::rtrim(std::string &s) {
    s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {
        return !std::isspace(ch);
    }).base(), s.end());
}
