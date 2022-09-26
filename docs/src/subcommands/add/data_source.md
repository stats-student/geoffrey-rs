<p align="center">
    <img src="../../static/images/geoffrey-logo.png" class="logo"/>
</p>

# geoff add data-source

```shell
geoff add data-source [OPTIONS] NAME
```

Adds a new folder in data_sources folder called `NAME` which contains a metadata markdown document to populate. The metadata file captures some details about the data source and any key people that were involved in the collection or authorisation to access the data.

## Data sources

There are three different metadata templates that are created depending on whether the data source is a 

<img src="../../static/images/db.png" height="25px" width="25px" style="vertical-align: middle;"> database source  
  
<img src="../../static/images/folder.png" height="25px" width="25px" style="vertical-align: middle;"> extract source  
  
<img src="../../static/images/cloud-download.png" height="25px" width="25px" style="vertical-align: middle;"> 
web download source

### Database source
A database source is used when you have access to the database that holds the data you're extracting. You may still export that data to some other file type or connect to the database directly.

### Extract source
An extract source is used when you are reliant on someone else in the business getting some data, potentially cleaning/transforming the data and sending it to you.

### Web download source
A web download source is used when you have downloaded some data from a public website e.g kaggle, government websites

## Arguments

`name`
The name of the data sources

Data source name

```shell
foo@bar:~$ geoff add data-source iris
```

If no options are passed a directory is created with an empty metadata.md

## Options

`--database\-d`
Creates a folder for a database data source 

`--extract\-e`
Creates a folder for an extract data source 

`--web-download\-w`
Creates a folder for a web download data source 

`--help`
Shows help message and exits

## Examples

Add a data source with no options

```shell
foo@bar:~$ geoff add data-source iris
🎯 iris data source added!

🖿 data_sources
└── 🖿 iris
    └── 🗋 metadata.md
```

Add a database data source

```shell
foo@bar:~$ geoff add data-source --database iris
🎯 iris data source added!

🖿 data_sources
└── 🖿 iris
    └── 🗋 metadata.md
```

Add a extract data source

```shell
foo@bar:~$ geoff add data-source --extract iris
🎯 iris data source added!

🖿 data_sources
└── 🖿 iris
    └── 🗋 metadata.md
```

Add a web download data source

```shell
foo@bar:~$ geoff add data-source --web-download iris
🎯 iris data source added!

🖿 data_sources
└── 🖿 iris
    └── 🗋 metadata.md
```