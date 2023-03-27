# AIDADB
Personal SUBD with own DB based on Rust.
The main goal of this project is to create personal Database with the strong security & fast responsing to requests with big data.

Now we have this query commnads:
<ol>
  <li><b>create</b></li>
  <li><b>use</b></li>
  <li><b>insert</b></li>
  <li><b>select(not stable)</b></li>
</ol>


# How to build and run:

To start use this DB u need to build the program using command:
```console
admin@aidadb:~$ cargo build
Finished dev [unoptimized + debuginfo] target(s) in 0.84s
Process finished with exit code 0
```
After that u need to run this app. For windows - aidb.exe, for Linux and MacOS - aidb.o


# How to send requests:

<b>Create database request:</b>
```console
> create <database_name>
Database created successfully
```

<b>Select database request:</b>
```console
> use <database_name>
Switched to database '<database_name>'
Current database path: (Windows)'C:\Users\<username>\.aidadb\databases\<database_name>.aidb' / (Linux or MacOs) '/home/<username>/.aidadb/databases/<database_name>.aidb'
```

<b>NOTE:</b> To use the next requests u need to select database in which u can add or from which u can get information!

<b>Insert in table request:</b>
```console
> insert <table_name> <key>=<value>
Inserted row into table <table_name>.
```
<b>NOTE:</b> U can create new tables using the same command; Row size of data is infinity.


<b>Select info from table request:</b>
```console
> select <table_name>
<info from table>.
```
