# 如何使用sea-orm

migration是用来定义，创建，更新数据库中表的。
sea-orm有一个sea-orm-cli工具，它可以根据migration中
的定义做数据库中建表。

当数据库中的表建好以后，可以通过sea-orm-cli的generate entity来根据数据中的定义来生成entity。

做本地跑一个postgreSQL的容器：
`docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres`

创建一个数据库
`create database bakeries_db;`

之后
` DATABASE_URL="postgresql://postgres:mysecretpassword@localhost:5432/bakeries_db" sea-orm-cli migrate refresh `

`sea-orm-cli generate entity -u postgresql://postgres:mysecretpassword@localhost:5432/bakeries_db -o src/entities`