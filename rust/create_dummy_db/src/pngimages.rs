use std::sync::Once;

use log::info;
use magick_rust::{magick_wand_genesis, MagickWand};
use rand::prelude::ThreadRng;
use rand::Rng;
use svg::node::element::Line;
use svg::{node, Document};

static START: Once = Once::new();

pub fn create_image(
    w: usize,
    h: usize,
    image_idx: usize,
    image_cnt: usize,
    filename: &String,
    rng: &mut ThreadRng,
    article_code: String,
) {
    let x1 = 0;
    let y1 = 0;
    let x2 = w;
    let y2 = 1 * h / 4;
    let line1 = create_line_node(x1, y1, x2, y2);

    let x2 = w;
    let y2 = 2 * h / 4;
    let line2 = create_line_node(x1, y1, x2, y2);

    let x2 = w;
    let y2 = 3 * h / 4;
    let line3 = create_line_node(x1, y1, x2, y2);

    let x2 = w;
    let y2 = h;
    let line4 = create_line_node(x1, y1, x2, y2);

    let x2 = 3 * w / 4;
    let y2 = h;
    let line5 = create_line_node(x1, y1, x2, y2);

    let x2 = 1 * w / 4;
    let y2 = h;
    let line6 = create_line_node(x1, y1, x2, y2);

    let x2 = 2 * w / 4;
    let y2 = h;
    let line7 = create_line_node(x1, y1, x2, y2);

    let cx = rng.gen_range(30..w / 2);
    let cy = rng.gen_range(30..h / 2);

    let scale: f64 = rng.gen_range(0.2..1.0);

    let path1 = node::element::Path::new()
        .set("style", "display:inline;fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 103.64846,296.69126 c 3.06613,-0.37767 6.15412,-0.52158 9.24002,-0.61215 2.8528,-0.0638 5.70636,-0.0717 8.55972,-0.0743 3.18352,-0.01 6.36704,0.004 9.55057,-0.005 3.97089,-0.0146 7.94173,-0.0189 11.91265,-0.0215 4.19805,0.006 8.3961,0.003 12.59415,-0.004 4.43945,0.30694 8.88156,0.53603 13.32909,0.68253 4.58186,0.0961 9.15917,-0.29312 13.73634,-0.47777 3.84293,-0.0297 7.6625,-0.22184 11.48627,-0.59097 2.01665,-0.21796 4.04188,-0.31455 6.06895,-0.3497 1.27389,-0.0131 2.54775,0.002 3.82163,0.003 1.08091,0.002 2.16188,0.004 3.24277,0.0122 0.95229,-0.006 1.90457,10e-4 2.85685,-8.5e-4 0.68839,0.002 1.37684,0.0124 2.06522,0.008 0.47666,0.006 0.95327,0.002 1.42992,0.005 0.24287,0.003 0.11309,0.003 0.38932,-8.8e-4 0,0 -12.19835,-8.86261 -12.19835,-8.86261 v 0 c -0.28255,-0.004 -0.15196,-0.004 -0.39175,-8.8e-4 -0.47164,0.004 -0.94324,-9.9e-4 -1.41488,0.005 -0.68083,-0.004 -1.36171,0.006 -2.04254,0.008 -0.95296,-0.002 -1.90592,0.005 -2.85888,-0.002 -1.07389,0.006 -2.14786,0.004 -3.22176,0.003 -1.27082,-0.002 -2.54212,0.008 -3.81198,0.061 -2.02035,0.0839 -4.0343,0.25551 -6.04179,0.50013 -3.77254,0.3999 -7.55256,0.50277 -11.344,0.59098 -4.54458,0.22541 -9.09541,0.59885 -13.64762,0.39845 -4.45919,-0.15686 -8.90808,-0.48562 -13.36644,-0.64774 -4.20167,-0.007 -8.40334,-0.01 -12.60502,-0.004 -3.99189,-0.003 -7.98369,-0.007 -11.97554,-0.025 -3.19295,-0.0122 -6.38591,-0.005 -9.57886,-0.0123 -2.86271,0.005 -5.72688,-0.006 -8.58752,0.11926 -3.137267,0.13925 -6.26867,0.35354 -9.396514,0.63547 z");

    let path2 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 212.0367,316.07598 c 0.98913,2.75175 1.73586,5.58396 2.63853,8.36549 0.43534,1.32537 0.86954,2.65395 1.14288,4.02328 0.14518,1.38617 0.0569,2.76953 0.29107,4.14618 0.3123,1.56151 0.45075,3.1471 0.55942,4.73322 0.002,2.2774 0.31998,4.52871 0.56513,6.78739 0.21843,2.60027 0.65975,5.17241 0.90093,7.77099 0.40006,2.57471 0.66148,5.16565 0.98547,7.74969 0.30497,2.39158 0.73738,4.76305 1.29768,7.10755 0.65512,2.34413 1.4407,4.64985 2.22523,6.95327 1.15542,3.01706 2.31511,6.02274 3.3781,9.07554 1.19292,3.47663 2.09873,7.03965 3.21944,10.53856 0.9551,2.91915 1.97724,5.81602 2.9633,8.72505 0.86119,2.48427 2.15865,4.77746 3.2329,7.17003 0.96169,1.994 1.33649,4.13838 1.52668,6.32201 0.24523,2.05692 0.58939,4.09553 0.75589,6.16167 0.13171,2.29482 0.13075,4.58522 0.37601,6.87237 0.38842,3.03956 1.31199,5.95954 2.33716,8.83435 1.13195,3.15381 2.57683,6.18108 3.9949,9.21322 0.55181,1.42063 1.39549,2.70315 1.9898,4.1017 0.24934,1.1995 0.52933,2.38926 0.80866,3.58473 0.21348,0.99137 0.28872,2.00102 0.33729,3.01122 -0.083,0.87834 0.12806,1.66997 0.39118,2.49575 0.14747,0.7384 0.65321,1.28082 0.99752,1.92027 0.10011,0.72212 0.34164,1.34703 0.62354,2.01052 0.25547,0.55901 0.35487,1.1755 0.39801,1.78456 0.0729,0.87105 0.43284,1.703 0.64999,2.55834 0.0978,0.51508 0.11854,1.02823 0.2716,1.53098 0,0 13.70584,6.23133 13.70584,6.23133 v 0 c -0.30185,-0.40394 -0.48658,-0.61757 -0.45733,-1.17121 -0.0849,-0.73104 -0.1714,-1.8846 -0.70483,-2.38241 -0.031,-0.73818 -0.0701,-1.46987 -0.21702,-2.19684 -0.0819,-0.25423 -0.12413,-0.52325 -0.2364,-0.7656 -0.19279,-0.41613 -0.55608,-0.55054 -0.44944,-1.11312 -0.0833,-0.81332 -0.34552,-1.45433 -0.94924,-2.02454 -0.27055,-0.72038 -0.66172,-1.32399 -0.60734,-2.09196 -0.013,-1.0857 -0.0742,-2.17432 -0.26012,-3.24546 -0.20086,-1.26446 -0.68475,-2.4324 -0.72895,-3.717 -0.34136,-1.61521 -1.37318,-2.87016 -2.00845,-4.37189 -1.50567,-2.97303 -2.96532,-5.98537 -4.13186,-9.10814 -1.03359,-2.75897 -1.91794,-5.56201 -2.41477,-8.4727 -0.24951,-2.26672 -0.32736,-4.52416 -0.3561,-6.80518 -0.14012,-2.07357 -0.37208,-4.13218 -0.77583,-6.17293 -0.14577,-2.31544 -0.36634,-4.63508 -1.31937,-6.78361 -1.03138,-2.39974 -2.30127,-4.66994 -3.31945,-7.07445 -1.01675,-2.91464 -2.00308,-5.83897 -3.04818,-8.74252 -1.17618,-3.48785 -2.0436,-7.06355 -3.19732,-10.55898 -1.06404,-3.02963 -2.03372,-6.08066 -3.31342,-9.02756 -0.76208,-2.27695 -1.5207,-4.55053 -2.20735,-6.85049 -0.71023,-2.26019 -1.07619,-4.6162 -1.42608,-6.95417 -0.39198,-2.55244 -0.48783,-5.14569 -1.01158,-7.68188 -0.22643,-2.60366 -0.63492,-5.18099 -0.89207,-7.77874 -0.19205,-2.23012 -0.61963,-4.43906 -0.52649,-6.68426 -0.0425,-1.64089 -0.21259,-3.27674 -0.47019,-4.89783 -0.24131,-1.36408 -0.36758,-2.64806 -0.2967,-4.04686 -0.0609,-1.5281 -0.64632,-2.93948 -1.11945,-4.37927 -1.00031,-2.74118 -1.8143,-5.54226 -2.54568,-8.36644 z");

    let path3 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 271.09198,503.34319 c 4.65116,0.19125 9.30228,0.3804 13.95485,0.53462 4.13284,0.10334 8.26751,0.0781 12.40123,0.0639 5.30921,-0.028 10.61846,-0.0225 15.92771,-0.0143 4.00913,0.0309 8.01839,0.0229 12.02757,0.0431 3.29214,0.0198 6.58425,9.2e-4 9.87636,-0.007 3.92241,-0.008 7.84477,-0.0113 11.76718,-0.004 2.67524,0.0114 5.35041,0.0354 8.02564,0.049 1.9869,4.2e-4 3.97379,-6.8e-4 5.96068,-0.002 1.52996,-0.002 3.05977,0.0209 4.58972,0.021 0.92205,0.01 1.844,0.0168 2.76602,0.003 1.72908,0.11465 3.40667,-0.20081 5.10324,-0.48041 1.39717,-0.23229 2.81294,-0.30418 4.22674,-0.33371 1.04468,0.10527 2.00328,-0.26604 3.00105,-0.51723 0,0 -12.34964,-8.53648 -12.34964,-8.53648 v 0 c -0.93802,0.2125 -1.86178,0.28866 -2.82457,0.28374 -1.44096,0.0847 -2.87592,0.2465 -4.29729,0.50159 -1.66413,0.25807 -3.32551,0.32985 -5.00901,0.25538 -0.91935,-0.0133 -1.83862,-0.008 -2.758,0.003 -1.51067,8e-5 -3.02121,0.0226 -4.53189,0.021 -1.98836,-9.2e-4 -3.97672,-0.002 -5.96509,-0.002 -2.63025,0.0136 -5.26044,0.0377 -7.89071,0.049 -3.9261,0.007 -7.85215,0.004 -11.77824,-0.004 -3.29868,-0.008 -6.59735,-0.0271 -9.89605,-0.007 -3.9682,0.0207 -7.9365,0.0131 -11.90464,0.0466 -5.3181,0.0114 -10.63622,0.0226 -15.95428,-0.007 -4.14191,-0.0211 -8.28408,-0.0379 -12.42518,-0.12964 -4.68423,-0.10026 -9.37575,-0.2305 -14.0388,-0.71302 z");

    let path4 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 427.79201,313.1604 c -0.79211,2.07316 -1.86987,4.00896 -2.86666,5.9838 -0.56342,0.95397 -1.20144,1.88199 -2.01936,2.63462 -0.6036,0.74997 -1.31494,1.40335 -1.94231,2.12892 -0.96275,0.88646 -1.77763,1.94298 -2.66263,2.91359 -1.20425,1.39869 -2.13638,2.99347 -3.19725,4.49702 -0.75855,1.25514 -1.42234,2.56312 -2.20958,3.80327 -0.62954,1.18453 -1.44595,2.25757 -2.19779,3.36619 -1.00151,1.43821 -2.13748,2.7659 -3.12003,4.21644 -0.92941,1.42063 -1.58665,2.93767 -1.99006,4.58205 -0.34968,1.75834 -0.71354,3.5045 -0.93069,5.28586 -0.23722,1.7706 -0.73829,3.48021 -1.20242,5.19907 -0.64103,1.95949 -1.09651,3.97221 -1.66084,5.95437 -0.85364,2.32132 -1.59623,4.68093 -2.3345,7.04086 -0.69322,2.34805 -1.2953,4.72324 -1.92572,7.08943 -0.60622,2.11717 -1.28939,4.2089 -1.87281,6.33388 -0.6118,2.22556 -1.12655,4.46701 -1.55312,6.73511 -0.50448,2.38096 -0.7846,4.80039 -0.96227,7.22526 -0.10051,1.60895 -0.13665,3.22053 -0.13946,4.83227 -10e-4,1.59826 0.0174,3.1964 0.0258,4.79459 -0.03,2.1033 0.36724,4.18002 0.57328,6.26736 0.17108,1.9484 0.2174,3.90494 0.2422,5.85979 -0.003,1.69775 -0.005,3.39537 -4.5e-4,5.09316 -0.0118,2.59091 0.0242,5.17274 0.24816,7.75455 0.25563,2.52455 0.34694,5.06077 0.70542,7.57376 0.29114,1.62708 0.43735,3.27313 0.64596,4.91083 0.27374,1.67964 0.73135,3.30586 1.41554,4.86262 0.87279,1.76062 1.7011,3.54194 2.50558,5.3346 0.87638,2.31086 2.00556,4.50333 3.08358,6.72254 0.84976,1.76657 1.72564,3.52065 2.61206,5.26913 0.90863,1.75483 2.01426,3.39647 3.14929,5.01031 1.0069,1.32309 2.11268,2.56701 3.12079,3.88838 0.88933,1.15946 1.79855,2.32738 2.43245,3.651 0.29084,1.1219 0.43777,2.23644 0.96281,3.28506 0.57084,1.24966 1.25616,2.44156 1.99987,3.59558 0.73877,1.39158 1.48636,2.77542 2.26331,4.1467 0.58185,1.03012 1.46637,1.84748 2.01618,2.89141 0.51597,0.62186 0.88956,1.3293 1.32358,2.0102 0.27303,0.31048 0.48382,0.59162 0.61835,0.99065 0.27938,0.5493 -0.28862,1.34765 0.56552,-0.34694 0,0 12.29655,8.76432 12.29655,8.76432 v 0 c 0.52345,-0.74752 0.76919,-0.99601 0.71474,-1.8219 -0.0523,-0.52377 -0.0923,-1.14772 -0.39323,-1.57355 -0.49494,-0.66362 -0.8697,-1.37553 -1.34406,-2.02803 -0.62769,-0.66762 -0.0401,0.0397 -0.53658,-0.88108 -0.30006,-0.5565 -0.7649,-0.9408 -1.14808,-1.41871 -0.14668,-0.18294 -0.24781,-0.39816 -0.37171,-0.59724 -0.99656,-1.18847 -1.65641,-2.67473 -2.3167,-4.07071 -0.65378,-1.16523 -1.46036,-2.23704 -2.01956,-3.45348 -0.47818,-1.02929 -1.11103,-1.88879 -1.04383,-3.10396 -0.34389,-1.56279 -1.31331,-2.87572 -2.28626,-4.1214 -0.99247,-1.34517 -2.05604,-2.61691 -3.07711,-3.9405 -1.33501,-1.38844 -2.265,-3.11419 -3.30579,-4.72431 -0.89584,-1.75927 -1.76741,-3.5235 -2.66647,-5.27682 -1.10171,-2.18297 -2.2605,-4.34315 -3.0826,-6.6524 -0.79244,-1.81095 -1.60342,-3.60572 -2.44007,-5.39464 -0.82102,-1.38091 -1.29942,-2.87902 -1.57524,-4.45957 -0.40614,-1.5946 -0.2942,-3.301 -0.7221,-4.90319 -0.42882,-2.49031 -0.55931,-5.0017 -0.81371,-7.51559 -0.23028,-2.52788 -0.30731,-5.0457 -0.20813,-7.58458 0.008,-1.70545 0.0147,-3.41064 -0.0101,-5.11604 -0.0121,-1.99377 -0.0332,-3.9886 -0.20223,-5.97677 -0.17362,-2.0496 -0.54845,-4.08147 -0.60806,-6.13383 0.005,-1.5915 0.0168,-3.18297 0.009,-4.77451 -0.002,-1.59316 -0.0215,-3.18702 0.044,-4.7793 0.11475,-2.3689 0.32355,-4.73765 0.83571,-7.05833 0.47998,-2.22074 0.88772,-4.45281 1.51772,-6.63884 0.58189,-2.12533 1.27503,-4.21491 1.8726,-6.33431 0.65804,-2.33923 1.17644,-4.71746 1.8733,-7.04595 0.7727,-2.35904 1.46229,-4.74676 2.3658,-7.06047 0.61018,-1.98129 1.05534,-4.00893 1.73483,-5.96788 0.45681,-1.79497 1.00818,-3.56032 1.29005,-5.39728 0.17842,-1.73565 0.5454,-3.42131 0.87837,-5.12819 0.30091,-1.48716 0.87631,-2.82304 1.72233,-4.08807 0.97988,-1.43856 2.05843,-2.80063 3.08348,-4.20751 0.81032,-1.10408 1.65741,-2.19507 2.28051,-3.41984 0.74791,-1.23207 1.42474,-2.50082 2.14207,-3.74789 1.00874,-1.45619 1.88647,-3.00815 3.08029,-4.32869 0.90748,-0.96651 1.77294,-1.97495 2.73849,-2.88455 0.65607,-0.67866 1.26842,-1.39466 1.91181,-2.08538 0.85089,-0.94272 1.62135,-1.94532 2.20129,-3.0815 0.96138,-2.04379 1.98718,-4.05757 2.9423,-6.10475 z");

    let path5 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 464.80326,322.6566 c 1.53083,2.26231 2.76128,4.73923 3.72412,7.29406 0.74278,1.98394 0.77908,4.10047 0.96934,6.18159 0.15858,2.01447 0.71673,3.96166 1.13302,5.9306 0.1239,1.07803 0.56572,2.06449 0.85529,3.0996 0.30471,1.24457 1.10663,2.29982 1.94263,3.244 1.08788,1.05177 2.08649,2.18582 3.16472,3.24642 0.81931,0.72473 1.66313,1.4074 2.43147,2.19157 0.48785,0.34565 0.77585,0.81458 0.81984,1.39306 0.56038,0.83485 0.70286,1.78376 0.80394,2.76316 0.0739,1.92747 0.17754,3.84263 0.44576,5.75409 0.0973,1.39633 0.373,2.58125 1.06602,3.79668 0.69128,1.01446 1.57782,1.86915 2.41824,2.75615 0,0 13.73914,5.7612 13.73914,5.7612 v 0 c -0.78015,-0.87968 -1.59365,-1.71604 -2.45187,-2.52064 -0.91551,-1.15569 -1.19986,-1.69948 -1.34754,-3.20583 -0.30523,-1.87537 -0.55104,-3.72783 -0.46516,-5.63513 -0.0503,-1.09911 -0.0345,-2.24484 -0.47213,-3.2732 -0.0718,-0.0147 -0.18213,0.0212 -0.21529,-0.0441 -0.0722,-0.1422 -0.1434,-1.63172 -0.45479,-1.75985 -0.76171,-0.80999 -1.47596,-1.73184 -2.45445,-2.27813 -1.11761,-1.02121 -2.11603,-2.10895 -3.17964,-3.17886 -0.83346,-0.79407 -1.58979,-1.57522 -2.20892,-2.54211 -0.28006,-1.04046 -0.67547,-2.02736 -0.9349,-3.05195 -0.44046,-1.93666 -1.05825,-3.84161 -1.28607,-5.81796 -0.25421,-2.16943 -0.12011,-4.39556 -0.81746,-6.49977 -0.88868,-2.67293 -2.21593,-5.14505 -3.49269,-7.64882 z");

    let path6 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 530.61712,309.06086 c 1.99992,4.87703 3.07784,10.0472 4.14306,15.18881 0.57074,2.88599 1.0036,5.79993 1.20117,8.73574 0.0841,1.33436 0.0539,2.67126 0.0409,4.00698 -0.0312,1.21519 -0.007,2.43087 0.0107,3.64606 0.1655,2.15275 0.47509,4.29012 0.64494,6.44365 0.098,1.5027 0.14441,3.00928 0.14381,4.51515 -0.005,1.14502 -0.002,2.28999 0.004,3.43502 -0.10034,0.75086 0.22122,1.65722 -0.21165,2.3316 -0.49907,0.4619 -0.53017,1.13623 -0.54632,1.77802 0.13139,0.74786 0.054,0.37433 0.23291,1.12045 0,0 13.52537,6.51871 13.52537,6.51871 v 0 c -0.1521,-0.72338 -0.0631,-0.38363 -0.25778,-1.02162 -0.29531,-0.44986 -0.18347,-0.46075 0.0949,-0.85878 0.5408,-0.98012 0.6166,-1.94702 0.52895,-3.05554 0.008,-1.14863 0.0202,-2.29721 0.005,-3.44584 -0.0242,-1.5467 -0.0364,-3.09355 -0.15772,-4.63685 -0.15612,-2.11803 -0.39324,-4.22841 -0.68172,-6.33251 0.0209,-1.20179 0.0479,-2.40366 0.045,-3.6057 0.01,-1.3796 7.3e-4,-2.75993 -0.0547,-4.13856 -0.19669,-3.05254 -0.77116,-6.06014 -1.37587,-9.05489 -1.09213,-5.13107 -2.18933,-10.27324 -3.8178,-15.26632 z");

    let path7 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 611.44324,320.66384 c -0.54282,2.2825 -0.79103,4.63081 -1.33868,6.90979 -0.33378,1.11378 -0.6836,2.21735 -0.8877,3.36359 -0.10202,0.57016 -0.14278,1.15286 -0.1292,1.72844 -0.2995,0.53754 -0.52106,1.10837 -0.66676,1.70491 -0.0288,0.7484 -0.29374,1.4126 -0.53213,2.1116 -0.25534,0.60693 -0.33763,1.24983 -0.42092,1.89598 -0.28972,0.96756 -0.39275,1.9862 -0.80873,2.91133 -0.41499,0.83391 -0.57872,1.73942 -0.76849,2.64025 -0.30652,0.83232 -0.52301,1.69088 -0.7584,2.544 -0.35532,0.75974 -0.62332,1.53868 -0.78476,2.36162 -0.13751,0.73623 -0.4482,1.43065 -0.68925,2.14294 -0.71592,1.20408 -1.55418,2.32594 -2.34537,3.48087 -0.99786,1.21989 -2.06919,2.39079 -2.99109,3.67001 -0.99944,1.1577 -1.88225,2.41108 -2.74683,3.67134 -0.81953,1.24937 -1.63318,2.48026 -2.31178,3.81332 -0.60631,1.2761 -1.19903,2.55107 -1.63776,3.89554 -0.25449,1.01636 -0.56685,2.02661 -0.95532,3.0004 -0.5053,1.35866 -1.24484,2.60026 -1.95946,3.85402 -0.87844,1.16401 -1.60797,2.4398 -2.49294,3.59558 -0.76323,1.03735 -1.33378,2.19246 -2.0266,3.27441 -0.81949,1.09469 -1.4294,2.32095 -2.10518,3.50422 -0.99038,1.31812 -1.8322,2.74458 -2.80741,4.07162 -0.7848,1.11034 -1.62472,2.18781 -2.47983,3.24793 -0.46518,0.55644 -0.79427,1.23395 -1.32906,1.73027 -0.8184,1.04339 -1.42081,2.22669 -2.06304,3.38077 -0.61263,0.98513 -1.18982,2.02168 -1.92287,2.9222 -0.46105,0.70131 -0.90701,1.40675 -1.4387,2.05715 -0.66043,0.66162 -1.51072,1.12558 -2.24271,1.71698 -1.01466,0.95318 -2.09012,1.83718 -3.10173,2.78937 -1.1298,1.02166 -2.14983,2.15608 -3.35523,3.09284 -1.08822,0.88174 -2.14168,1.80647 -3.15613,2.77211 -1.03494,1.36082 -2.17073,2.64203 -3.20805,3.99985 -0.91428,1.29362 -2.26571,2.10445 -3.39667,3.16356 -1.06453,0.83237 -2.1186,1.66262 -3.21436,2.45225 -1.48444,1.24112 -3.02781,2.40086 -4.54741,3.59818 -1.77214,1.26338 -3.65516,2.36627 -5.4998,3.52115 -1.69307,1.1174 -3.29001,2.32728 -4.75067,3.73476 -1.405,1.27767 -3.03251,2.2697 -4.43124,3.54645 -1.64012,1.54552 -2.90639,3.45953 -4.05513,5.38768 -0.69269,1.1396 -1.4366,2.1989 -2.28947,3.21431 -0.80078,1.0082 -1.74644,1.88923 -2.51763,2.9191 -0.71288,0.87712 -1.53853,1.55629 -2.54872,2.05013 -0.82951,0.10712 -1.52936,0.54988 -2.36012,0.71095 -1.11455,-0.0287 -2.09176,0.39414 -3.14444,0.6742 -0.92889,0.13076 -1.72565,0.47743 -2.56604,0.87722 -0.96747,0.31652 -1.68293,1.28182 -2.3843,1.99906 -0.40008,0.40831 -0.18974,0.25442 -0.60787,0.49174 0,0 12.71696,8.03114 12.71696,8.03114 v 0 c 0.51497,-0.46161 0.28087,-0.23055 0.70836,-0.68698 0.62704,-0.63962 1.22372,-1.25037 2.10237,-1.51089 0.78167,-0.32638 1.61663,-0.44049 2.41979,-0.71082 1.03825,-0.30959 2.10599,-0.37071 3.1498,-0.66152 0.87345,-0.30485 1.76471,-0.58572 2.57961,-1.02056 1.0748,-0.70583 2.01518,-1.57071 2.82597,-2.57138 0.78612,-0.98692 1.69669,-1.87077 2.48858,-2.85465 0.92135,-1.0776 1.64181,-2.25506 2.36828,-3.472 1.03169,-1.81203 2.21568,-3.58859 3.78996,-4.9747 1.48099,-1.21184 3.083,-2.2757 4.48088,-3.59165 1.41152,-1.30974 2.94899,-2.4583 4.57417,-3.49256 1.8893,-1.17323 3.78958,-2.34013 5.58816,-3.64902 1.5205,-1.16962 3.0147,-2.37139 4.54834,-3.52262 1.14619,-0.80805 2.24598,-1.6797 3.31424,-2.58822 1.24583,-1.0629 2.52822,-2.07671 3.50215,-3.41885 1.00264,-1.28681 2.09152,-2.50766 3.12787,-3.76719 1.00615,-0.92882 2.02343,-1.84554 3.10133,-2.69158 1.16891,-1.01269 2.23549,-2.13397 3.38749,-3.16574 1.01258,-0.89717 2.01863,-1.80459 3.06696,-2.65945 0.8592,-0.62443 1.78396,-1.20209 2.47044,-2.02275 0.50568,-0.67561 0.96165,-1.3773 1.41962,-2.08519 0.72914,-0.99488 1.38103,-2.04986 1.98027,-3.12837 0.60352,-1.04666 1.15763,-2.13386 1.95858,-3.0489 0.49477,-0.59005 0.94887,-1.20601 1.36669,-1.85075 0.8244,-1.10553 1.69695,-2.17787 2.46405,-3.32356 0.97479,-1.39165 1.88874,-2.82616 2.90571,-4.18878 0.67178,-1.16491 1.26291,-2.37861 2.09341,-3.44621 0.65014,-1.0652 1.21893,-2.17636 1.96327,-3.18186 0.8743,-1.23388 1.66447,-2.52909 2.54025,-3.76399 0.74719,-1.29858 1.5452,-2.57215 2.09849,-3.97077 0.45401,-1.00103 0.75678,-2.09744 1.03176,-3.15666 0.34629,-1.28311 0.90418,-2.46953 1.48989,-3.65882 0.66169,-1.26464 1.37363,-2.48391 2.18096,-3.66438 0.87185,-1.24359 1.78474,-2.45556 2.74777,-3.63051 0.94247,-1.27802 2.00953,-2.46271 2.98898,-3.71277 0.87299,-1.24744 1.81346,-2.44837 2.57513,-3.76969 0.32093,-0.77807 0.6395,-1.60074 0.86638,-2.39647 0.0874,-0.75692 0.29731,-1.45723 0.64389,-2.14055 0.33198,-0.84894 0.4212,-1.77501 0.79201,-2.61765 0.27131,-0.80409 0.28052,-1.67073 0.66855,-2.44428 0.45629,-1.00668 0.63942,-2.05261 0.92837,-3.11519 0.21387,-0.60768 0.18727,-1.21735 0.3907,-1.83024 0.23305,-0.75214 0.65409,-1.46089 0.63645,-2.26115 0.0743,-0.55156 0.27118,-1.05079 0.56665,-1.52712 0.27568,-0.65154 0.2029,-1.28433 0.24424,-1.98576 0.12014,-1.12095 0.42661,-2.1974 0.74709,-3.27632 0.56286,-2.28502 0.64223,-4.6781 1.35092,-6.93758 z");

    let path8 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 656.47929,312.89922 c 1.19727,2.315 2.27292,4.68613 3.23224,7.10846 0.76633,1.70926 0.98631,3.57243 1.46292,5.36414 0.43877,1.59243 0.69503,3.22763 1.02378,4.84557 0.22208,1.66211 0.75533,3.21911 1.5985,4.66248 0.89526,1.40735 2.14411,2.4877 3.3046,3.65846 1.12648,1.0305 2.10723,2.20927 3.12022,3.3491 1.0489,1.21723 1.73721,2.65723 2.33183,4.13563 0.5184,1.63139 0.73776,3.34321 1.04543,5.02469 0.31253,1.50342 0.42152,3.03337 0.48028,4.56415 0.0322,1.79966 0.007,3.59271 0.22973,5.38124 0.14359,1.68135 0.56227,3.29452 1.04768,4.90254 0.70803,1.93975 1.6107,3.7992 2.38987,5.71095 0.83025,1.9005 1.43378,3.86063 1.89235,5.87776 0.59176,2.70065 1.01213,5.43546 1.48407,8.15877 0.75608,3.55894 1.59384,7.09879 2.18895,10.68929 0.46065,2.47009 0.96249,4.92826 1.21563,7.42995 0.34425,3.38056 -0.009,6.77114 -0.22774,10.14738 -0.21866,4.08489 -0.26789,8.17674 -0.34286,12.26619 0.0294,3.4781 -0.0681,6.94153 -0.43618,10.40079 -0.49163,3.82409 -1.20177,7.61812 -1.84683,11.41923 -0.68111,3.31616 -1.70286,6.54971 -2.57522,9.81869 -0.85465,3.3396 -2.2402,6.49664 -3.83977,9.53846 -1.37023,2.34221 -2.87862,4.60803 -4.05836,7.05527 -0.80592,1.76885 -1.81681,3.42549 -2.80926,5.09171 -1.03787,1.52881 -1.7172,3.25214 -2.49861,4.91597 -0.70402,1.85946 -1.36356,3.74346 -1.8381,5.67613 -0.44439,1.398 -0.19304,2.95357 -0.71458,4.31914 -0.40854,1.11196 -1.24575,1.90266 -2.0031,2.77143 -0.0849,-0.58337 -0.98582,1.94587 -0.92262,1.46737 0,0 12.94822,7.72844 12.94822,7.72844 v 0 c 0.31796,-0.48433 0.39509,-1.03554 0.8487,-1.44462 0.83682,-1.00056 1.7331,-1.9564 2.21953,-3.19464 0.63021,-1.4631 0.51625,-3.00536 0.79147,-4.55746 0.39594,-1.84498 1.02689,-3.62657 1.68423,-5.39183 0.77374,-1.61689 1.40517,-3.31201 2.42187,-4.801 1.01005,-1.71293 2.05359,-3.40812 2.86008,-5.23183 1.1073,-2.45893 2.60983,-4.70732 3.94056,-7.04713 1.62713,-3.14642 3.0631,-6.39264 3.97815,-9.82557 0.91879,-3.30744 1.92103,-6.59933 2.62084,-9.96116 0.63615,-3.82697 1.34996,-7.6433 1.86841,-11.48809 0.45892,-3.4992 0.64913,-7.00331 0.53028,-10.5333 0.003,-4.0676 0.0509,-8.137 0.3005,-12.19811 0.2137,-3.42442 0.62598,-6.84956 0.43986,-10.28413 -0.17092,-2.56508 -0.6565,-5.07268 -1.24858,-7.57346 -0.566,-3.59757 -1.4166,-7.13906 -2.15779,-10.70259 -0.50424,-2.71966 -0.86758,-5.46412 -1.36552,-8.18483 -0.4882,-2.06359 -0.86017,-4.14236 -1.82991,-6.06085 -0.65612,-1.93073 -1.52179,-3.77092 -2.36913,-5.62307 -0.51861,-1.53116 -1.01953,-3.06865 -1.17179,-4.68442 -0.2822,-1.73777 -0.32243,-3.45713 -0.27953,-5.21823 -0.0336,-1.60094 -0.0854,-3.20833 -0.3876,-4.78568 -0.34906,-1.73779 -0.6125,-3.48182 -0.94514,-5.22007 -0.48539,-1.60139 -1.14993,-3.20041 -2.12481,-4.56497 -0.86955,-1.30416 -2.0579,-2.35963 -3.02327,-3.59438 -1.58112,-1.30321 0.39788,0.3801 -1.16436,-1.12968 -0.77213,-0.74619 -1.71429,-1.29844 -2.25227,-2.27499 -1.09109,-1.10562 -1.64438,-2.53017 -1.89611,-4.05226 -0.30657,-1.65575 -0.63679,-3.29695 -0.94333,-4.94949 -0.55655,-1.82904 -0.80285,-3.72084 -1.46611,-5.52358 -0.94757,-2.44814 -2.04514,-4.82176 -3.13004,-7.2103 z");

    let path9 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 716.99965,503.03722 c 4.3405,-0.46199 8.71073,-0.55785 13.07208,-0.63051 3.11243,-0.0768 6.20072,-0.51324 9.31216,-0.68853 3.67707,-0.0377 7.33303,-0.34765 10.99825,-0.61175 3.9928,-0.52908 8.00157,-0.87892 12.02389,-1.0679 3.06016,-0.12285 6.12308,-0.12091 9.1851,-0.12574 2.6726,0.0506 5.32052,0.47636 7.99164,0.64086 2.44665,0.15253 4.89837,0.18171 7.34893,0.20516 2.1466,0.0215 4.29319,-0.007 6.43979,-0.009 2.85119,-0.0774 5.67839,-0.034 8.51303,0.2726 2.49004,0.33649 4.99916,0.44469 7.5085,0.50101 2.10135,-0.0954 4.13319,0.12473 6.18705,0.53142 1.23143,0.21861 2.47929,0.33447 3.7292,0.35937 1.11497,0.0113 2.22989,-0.001 3.34485,-0.008 1.52581,-0.021 3.04669,0.0528 4.55709,-0.17867 0,0 -12.18048,-8.66147 -12.18048,-8.66147 v 0 c -1.50399,0.0684 -3.00823,0.0248 -4.51337,0.0238 -1.10737,-0.005 -2.21615,-0.003 -3.32187,-0.07 -1.19991,-0.0984 -2.39367,-0.24767 -3.57816,-0.46681 -2.07471,-0.29215 -4.15347,-0.29988 -6.24633,-0.32103 -2.48241,-0.0982 -4.9568,-0.28566 -7.4268,-0.55547 -2.87242,-0.2256 -5.74808,-0.18072 -8.62768,-0.18361 -2.15141,7.3e-4 -4.30276,-0.0163 -6.45417,-0.0192 -2.41655,-0.02 -4.83381,-0.0684 -7.24481,-0.2456 -2.68127,-0.22252 -5.35607,-0.63135 -8.05331,-0.5478 -3.05655,0.0128 -6.11454,0.0508 -9.16645,0.23322 -4.01893,0.24807 -8.01636,0.69049 -12.02052,1.10395 -3.67813,0.2715 -7.35624,0.43122 -11.04154,0.56122 -3.08092,0.20935 -6.14807,0.58596 -9.23534,0.68249 -4.43452,0.17988 -8.87269,0.30952 -13.30071,0.61711 z");

    let path10 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 836.96337,474.9763 c 0.36116,-7.58952 1.88818,-15.06005 3.17231,-22.53144 1.10027,-6.23835 2.27964,-12.46663 3.25348,-18.72634 0.65676,-3.71243 1.0835,-7.45744 1.84792,-11.15068 1.00837,-3.61077 2.521,-7.06004 4.19644,-10.4055 1.21448,-2.61541 2.71745,-5.06337 4.29188,-7.47128 1.98357,-3.07363 3.76476,-6.26904 5.52465,-9.47398 2.06753,-3.86172 4.01449,-7.78995 5.70582,-11.83148 1.28059,-3.23521 2.00891,-6.63074 2.68668,-10.03073 0.6172,-2.99643 1.37383,-5.96343 2.25803,-8.89175 1.0154,-3.32576 1.88503,-6.69249 2.63582,-10.08747 0.5471,-2.55536 1.38081,-5.03403 1.99857,-7.57052 0.41834,-1.8718 0.86044,-3.72576 1.1154,-5.62907 0.29778,-2.08088 0.0979,-4.16073 -0.15909,-6.23075 -0.1687,-1.10287 -0.27509,-2.21123 -0.33037,-3.32492 -0.0397,-1.04979 -0.0309,-2.10065 -0.0322,-3.15097 0.0106,-0.85726 0.004,-1.71459 0.019,-2.57179 0.007,-0.4491 0.005,-0.89816 0.0171,-1.34725 0.0496,-0.55186 -0.0717,0.0705 -0.075,0.077 0,0 -12.5147,-8.47333 -12.5147,-8.47333 v 0 c -0.31033,0.47817 -0.87185,0.94041 -0.85933,1.54362 0.0123,0.45422 0.01,0.90839 0.016,1.36263 0.0136,0.86222 0.005,1.7245 0.0116,2.58678 -0.004,1.06575 -0.004,2.1315 0.005,3.19722 0.0229,1.17206 0.0944,2.34138 0.29917,3.49761 0.33387,1.94994 0.66887,3.90573 0.36892,5.88629 -0.20774,1.86662 -0.65013,3.66831 -1.03011,5.50218 -0.54442,2.57168 -1.38139,5.06873 -1.90859,7.64564 -0.67758,3.36605 -1.58225,6.67609 -2.58617,9.95857 -0.94336,2.96739 -1.69693,5.98679 -2.32601,9.03619 -0.63625,3.31809 -1.32402,6.62981 -2.49503,9.80912 -1.59209,4.02237 -3.53925,7.8869 -5.59364,11.6915 -1.77251,3.15749 -3.57672,6.29796 -5.59186,9.30871 -1.61526,2.45412 -3.11855,4.97567 -4.30376,7.67145 -1.69874,3.47048 -3.26496,7.0218 -4.41122,10.71705 -0.78184,3.73221 -1.1559,7.52661 -1.79113,11.28449 -0.83987,6.30952 -1.82442,12.60198 -2.89326,18.87632 -1.25049,7.46109 -2.77646,14.87781 -3.73886,22.3847 z");

    let path11 = node::element::Path::new()
        .set("style", "fill:#000000;stroke-width:7.80472;stroke-dasharray:none")
        .set("d", "m 885.11649,293.10069 c 5.92523,0.5231 11.90851,0.38769 17.85664,0.41523 6.11776,0.0414 12.23555,-0.002 18.35328,-0.0218 5.50679,0.0197 11.00507,0.0951 16.50367,-0.22844 4.41243,-0.31283 8.83366,-0.43761 13.25552,-0.5076 5.35089,-0.0821 10.70358,0.0139 16.05135,-0.19695 4.32945,-0.20635 8.65756,-0.4399 12.99101,-0.54256 1.64782,-0.0406 3.29614,-0.0316 4.94427,-0.0345 1.24598,0.004 2.49206,0.002 3.73796,0.0178 1.00158,0.003 2.0031,2e-4 3.00467,-0.005 0.73663,0.003 0.36706,0.002 1.10872,0.002 0,0 -12.14982,-8.82743 -12.14982,-8.82743 v 0 c -0.74129,-8.4e-4 -0.37316,-0.001 -1.10439,10e-4 -1.00663,-0.006 -2.01324,-0.009 -3.01989,-0.009 -1.23669,0.0127 -2.47348,0.004 -3.71021,0.0105 -1.62305,0.006 -3.24413,0.0689 -4.86488,0.15672 -4.25551,0.22491 -8.50869,0.48985 -12.76508,0.69589 -5.37425,0.13978 -10.75036,0.0128 -16.12569,0.11367 -4.43876,0.0751 -8.8757,0.2057 -13.30317,0.55008 -5.43495,0.31983 -10.87704,0.2306 -16.31944,0.2584 -6.10723,5.5e-4 -12.21441,9.1e-4 -18.32158,0.0354 -6.00733,-0.0154 -12.10427,0.17834 -18.03459,-0.90258 z");

    let g = node::element::Group::new()
        .add(path1)
        .add(path2)
        .add(path3)
        .add(path4)
        .add(path5)
        .add(path6)
        .add(path7)
        .add(path8)
        .add(path9)
        .add(path10)
        .add(path11)
        .set(
            "transform",
            format!("(translate({}, {}) scale({})", cx, cy, scale),
        );

    let t = node::Text::new(format!(
        "img_{}_{:02}_{:02}",
        article_code,
        image_idx + 1,
        image_cnt
    ));

    let txt = svg::node::element::Text::new()
        .set("x", 100)
        .set("y", 100)
        .set("stroke", "red")
        .set("stroke-width", 3)
        .set("font-size", "80px")
        .set("fill", "blue")
        .add(t);

    let document = Document::new()
        .set("viewBox", (0, 0, w, h))
        .add(g)
        .add(txt)
        .add(line1)
        .add(line2)
        .add(line3)
        .add(line4)
        .add(line5)
        .add(line6)
        .add(line7);

    let path = env!("CARGO_MANIFEST_DIR");
    let full_path = format!("{}/images/svg/{}.svg", path, filename);
    info!("path {}          full_path {}", path, full_path);
    svg::save(full_path, &document).unwrap();
    convert_to_png(filename, w, h);
}

fn create_line_node(x1: i32, y1: i32, x2: usize, y2: usize) -> Line {
    let line1 = node::element::Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", "black")
        .set("stroke-width", "2");
    line1
}

fn convert_to_png(filename: &str, w: usize, h: usize) {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let path = env!("CARGO_MANIFEST_DIR");
    let input = format!("{}/images/svg/{}.svg", path, filename);
    //     info!("path {}          input {}", path, input);
    let wand = MagickWand::new();
    wand.read_image(&input).expect("should find it");
    wand.fit(w, h);
    let output = format!("{}/images/png/{}.png", path, filename);
    //  info!("path {}    input {}   output {}", path, input, output);
    let x = wand.write_image(&output);
    match x {
        Ok(()) => {}
        Err(e) => println!("file save crashed  {}", e),
    }

    // let img = image::io::Reader::open(output).expect("should be able to read the PNG using image");
    let img = image::open(&output)
        .expect("should be able to read the PNG using image")
        .into_rgb8();

    // println!("dimensions {:?}", img.dimensions());
    // println!("{:?}", img.);

    // let output = format!("{}/images/png/{}_new.png", path, filename);

    image::save_buffer_with_format(
        &output,
        &img.into_raw(),
        w as u32,
        h as u32,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .expect("saving as RGB works");
}
