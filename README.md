# Welcome to Large File Windowed Search using Rust (lfws-rs)

## Install
```shell
cargo install --git https://github.com/jdr0887/lfws-rs
```

## Running
```shell
lfws-rs -f <large_file> -r <regex_search_string>  
```
## Example
In this example, the train.csv file is 170MB & comes from the Kaggle Foursquare Location Matching Challenge.
```
$ RUST_LOG=info lfws-rs -f train.csv -r "Jumbo Seafood Restaurant" -s 5
------------ 1 ------------
E_c22f871dc2bf1f,七十七銀行 本店営業部,38.260132423850585,140.87589596756771,青葉区中央3-3-20,仙台市,宮城県,980-8777,JP,https://www.77bank.co.jp,0222671111,Banks,P_e80f7aef7ac25e
E_c22f880a281b93,Karaca Cafe,41.050298902048816,28.95987782414901,Beyoğlu Okmeydanı,,,,TR,,,Hookah Bars,P_5234ae056a34cb
E_c22f895e64eac9,Taman Laut Jumbo Seafood Restaurant,-6.613927323729825,106.8127188211835,Jl.Pajajaran No.50A,Bogor,,,ID,,,Restaurants,P_db2bf5c1a4e750
E_c22f957642ab30,ALK Sala A11,52.272015,21.014946,,,,,PL,,,College Auditoriums,P_a7af2f538653bc
E_c22fb62c8b4fee,ABSHER United Company,24.80647,46.660446,شركة أبشر المتحدة,Riyadh,Riyadh,13322,SA,,,Business Centers,P_256b3720151a52
------------ 2 ------------
E_cd24c82c89a571,Mountain Resort : Sunset Beach,6.494340148375912,99.30499607063808,Lipe Island,Koh Lipe,Satun,,TH,,,Resorts,P_9cbc1d1a6044a3
E_cd24d05ea9e06a,Stasiun wates,-7.8850910355896255,110.121905255585,,,,,ID,,,,P_bb4dc0453ef58d
E_cd24ddf9167ed7,Taman Laut (Jumbo Seafood Restaurant),-6.61102991256688,106.81154429912569,Jalan Raya Pajajaran No. 50A,Bogor,Jawa Barat,16143,ID,,2518336623,Seafood Restaurants,P_db2bf5c1a4e750
E_cd24e009bea535,SEPHORA,53.485073598583526,-113.51344985680292,"5015- 111 Street NW, Unit 29",Edmonton,AB,T6H 4M6,CA,https://www.sephora.com/happening/stores/edmonton-southgate-centre,+17806381642,"Cosmetics Shops, Perfume Shops",P_68ad500f50bd48
E_cd24e90617420f,SEGi College Subang Jaya,3.0615541795559,101.59360161725016,Persiaran Kewajipan USJ 1,Subang Jaya,Selangor,47600,MY,http://www.segi.edu.my,386001888,"General Colleges & Universities, College Academic Buildings, College Classrooms",P_9609bda45a9281
------------ 3 ------------
E_fff6a6945004fe,Салон продаж Билайн,58.59627160863365,49.66264486312866,"Лепсе, 67",Киров,Россия,,RU,,,Electronics Stores,P_59972265fd5f0e
E_fff6ade227cc17,ビナウォーク 4番館,35.452804659117035,139.39460635185242,中央1-4-1,海老名市,神奈川県,243-0432,JP,https://www.odakyu-sc.com,046-236-3709,Shopping Malls,P_fc99fb79afe1c0
E_fff6b01f713e51,Jumbo Seafood Restaurant,3.5952644177148283,98.67603793516844,"Jl. Putri Hijau No. 8 A,B,C,D",Medan,Sumatera Utara,,ID,,0614525653,Seafood Restaurants,P_84e888f692ba40
E_fff70d498e1fbf,Taco Bell,32.81083062639929,-115.5703895025414,1990 N Imperial Ave,El Centro,CA,92243,US,http://www.tacobell.com,7604825697,"Fast Food Restaurants, Mexican Restaurants",P_215cf531d36c47
E_fff71952530e9d,Foodcourt,49.21709253035756,16.60806655883789,Cimburkova 593/4,Brno,Jihomoravský,612 00,CZ,,,Food Courts,P_6b9c48f51715f3
[2022-08-19T16:12:23Z INFO  lfws_rs] Duration: 310ms 880us 396ns
```
