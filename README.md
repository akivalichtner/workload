# Motivation

I started this project to motivate myself to learn the Rust programming language. The project is ambitious, which means it may not get done, but as long as it motivates me to code in Rust in my view it will have achieved its goal. I also think that when a project is based on a sound theoretical foundation, then the project or its concepts will find applicability or imitators. In this README I hope to show that such a foundation exists.

# Background

I have about 25 years' experience in IT. I am a physicist by training but have a career in IT. For the first 15 years of my career I developed mostly in Java(TM). I did a lot of work around Test-Driven Development, even building an exceptionally sophisticated end-to-end testing product. Eventually it became apparent that within my organization large codebases would always generate a long bug list, and I decided that my creative talent would most productively be employed in production support. I thus moved to a production support position, where I made a large impact. During this time wrote an original pure-Java tool that provides for the JVM what DTrace provides for Solaris. This tool was fun to write but ultimately fared no better than DTrace itself, being exceptionally capable but used by few. I also started learning about database server software, and discovered on my own skin that database performance is a hard nut to crack, even from a theoretical standpoint. Because of this experience I believe that I understand how sophisticated database software works, what it can and cannot do, what it expects from the database administrator, and how such software fares in practice. With the present project I am therefore in the position of a user who is engaged in building the original software he would like to use on the job in the first place. Lately I am moving back into the development space, and I think that building such a product in Rust can get me some modern development skills. I believe that Rust is a wisely-designed programming language which "checks all the boxes" and can become the new "default language" for building real software. It is especially well suited for system software such as a database server.

# Acceptance Criteria

For this project to move forward it needs to provide some value. There are already many database products. They have the advantage of being commercial products, or of being popular open source projects. Some of their features are quite sophisticated, and well tuned. As a friend of mine pointed out, a brand-new open-source database project in the best case scenario will just be a slow verion of older software. I believe the answer is that there is space for a better database product because the older software either makes assumptions about workload characteristics, or is too expensive to use, or does not scale horizontally, or is frustrating to optimize, or has its hands tied when innovating because of the long trail of customers that it needs to serve. If the present project boldly attacks these problems, then it deseves to exist.

# Principles

- SQL interface.

- Multi-version 2-phase locking.

- Designed for the cloud. The software needs to have adapters for the big cloud vendors and provision hardware where appropriate. The software needs to be designed to run on a "normal" class of machine, to achieve a good total cost of ownership.

- The best way to build a database server depends on the workload. Applications have various workloads. They are not OLTP or OLAP. Each application's workload is unique. There is no such thing as a good model of an application's workload, in any application space. The workload changes from application release to application release, and even all day long.

- Optimal data layout depends on the operation to be optimized, so in general data needs to be replicated with different layouts for various operations to be optimized properly. In some cases colunns in the same table must be stored in different layouts, in some cases rows in the same table must be stored in different layouts. The software should allow the user to specify layout based on predicates. In some cases data is best stored on disk like a traditional database, in other cases it should be stored in memory and replicated for availability.

- The software should be able to replicate data in order to scale horizontally.

- The software should provide options for primary/backup as well as active/active availability.

- The software should support massively parallel queries.

- The role of the wizard DBA is disappearing. The software cannot just provide some low-level features and rely on the DBA to hunt down slow SQL statements, formulate hypotheses, and experiment with solutions. In practice this is so burdensome for application developers that it is rarely attempted. The software should do the heavy lifting while leaving an unsophisticated DBA with the power to make decisions. The database software runs the queries and should explain to the user what the application has been doing with the database, how it's been going, and present options for improving SQL statement performance, explain the options in English, estimate their impact on other SQL statements and their likely financial cost. It should then allow the user to decide what course of action to pursue, implement the change with a click, and roll that decision back if the results are not satisfactory.

- Applications re-use code heavily. The system is doing the same operations all the time with different parameters and against different data. It should use every running SQL statement to understand the data better. It should do so continuously because the data content changes all the time. The traditional statistics collection approach inevitably produces query plans whose performance is not predictable. Fancy algorithms do not help when the software does not know the data. Statistics should be collected on all relational expressions (including sub-expressions,) not just on tables. This should happen automatically, all the time.

- The software will have built-in profiling which is always on. This is the only way for users to know where a query is truly spending its time. Application developers have a hard time keeping track of where transactions begin and end and even what they do, whereas the database knows all this information. For this reason the software will profile all SQL statements and all transactions.

- At installation time the binary should be given access to one of the supported cloud vendors, and should provision servers and install the software on them. It should then provide the user (at the console) with URLs for the administration tool, and a connection string for use with the driver software.

# User Interfaces

There should be a REST API that will provide access to all functions. There will also be a browser-based administration tool, and driver software for various languages, all of which will interact with the system through the API. The API will use HTTPS, and the API payload will be text-based. These choices are designed to make the software acceptable to modern firewalls (including web application firewalls), which inspect protocols. The format of the payload will be JSON because this is an accepted payload format today, and in the future additional formats could be supported.

# Architecture

The system will be implemented as a set of compute instances and network-attached storage. It will be the task of the software to deploy itself over the available instances.

There will be a service to run the API on port 443. This will scale across availability zones and regions. The system will manage its own DNS hosted zone and keep it updated. When needed, the API service will direct the client to a specific address or set of addresses, and the client will be required to comply. This functionality will be built into the driver software. The API service will allocate a session and provide the client with a session key. This session key is not specific to an HTTPS connection, AZ, or region. The client will provide the session key when using the API service. The system will time out the session after a configurable amount of time. When the session is timed out any uncommitted transactions will be automatically rolled back.

There will be a service to run the console on port 443. This will scale across availability zones and regions. The system will manage its own DNS hosted zone and keep it updated. If the console crashes the user will close the browser and log back in. The console service will allocate a session. If the console crashes this session will be lost. The console will be designed so as to keep the impact of a loss of session small.

There will be a system data service. The purpose of this service is to manage metadata about the database, including all database objects (schemas, tables, columns, indexes, constraints,) query expression statistics, users, privileges, metrics, etc.

There will be a service to read and write data. The purpose of this service is to provide an interface to access rows and columns. There will be several implementations of this service. There will be a uniform API for this service, but the internals will vary because of the conflicting requirements of performance, scalability and availability. The choice of implementation used will depend on the table, rows and columns to be accessed. One incarnation will consist of a process maniputlating data stored on a disk volume, another will be a replicated in-memory store. What is common to all the incarnations is ACID transactions with multi-version 2-phase locking. The data service will keep lock queues and grant row locks. It will also keep row histories for every row and garbage collect them as transactions commit or rollback, using a system change number. Rows will have row ids that will not depend on the specific data service instance.

There will be a query planner service. The purpose of this service is to be given a SQL query and to return to the client a query plan.

There will be a service to run query plans. The purpose of this service is to execute a query plan. This service is recursive in nature. Depending on the algorithms in the query plan and the constraints of CPU and memory available across the system, it will invoke other instances of itself. The instructions for these invocations will be included in the query plan itself. This service will be invoked by the API service.

The services will interact using TCP connections. Replication of in-memory data, concurrency control, and other features will rely on the totem multiple ring protocol. Replicas will be arranged in token-passing rings, and rings will be connected via gateways. This protocol will provide ring membership and topology (the full view of all the ring memberships and gateways) and message ordering. There will be a total order of messages, every message will have a unique timestamp. Not all messages will be addressed at every ring. The performance of the system depends on the system's ability to restrict most messages to a single ring, and limit forwarding of messages as much as possible. Still, in the general case a table may be stored using multiple data representations, which will require multiple rings. Delivery guarantees include agreed order (the case just described) or safe order. In safe order a processor delivers a message to the application only when it knows that every other processor already has the message. Safe delivery will be used sparingly. For database writes it is not necessary because if there is a failure we can just roll a transaction back.

# How Services Work Together

In this section I will discuss how the aforementioned architecture enables users to get work done in the database.

An application loads the driver software with the connection string. The connection string contains a DNS record that returns one of a list of IP address, corresponding to the API service.

The driver calls the API service and requests a session ID and a transaction ID. The API service delegates this to the system data service. The system data service sends out a totally ordered message and waits to receive it. The timestamp assigned to the message is used as the required ID. These messages (one for the session and one for the transaction) need to be sent in safe order (see above where the term was defined) but the destination of the messages does not have to go beyond the system data service, even though the IDs will be referenced elsewhere in the system later.

The driver calls the API service and sends it its session id and transaction id, and a SQL statement. The API service calls the query planner and receives a plan for the query. It then calls the query runner service.

The query runner service processes the next step in the query plan. The processing of query steps will be described elsewhere. When the query runner needs to read or write data, it sends a message to a data service. Read messages behave differently than write messages. Read messages are directed at a specific data service instance which is specified in the query plan. For example, one plan may scan a column-oriented store, while another a row-oriented store. There will be copies of the same store as well. When the data service instance scans a row it needs to assign a logical timestamp to the read, so that client can read the same row from other data service instances and get a consistent response. The query runner delegates the generation of the logical timestamp to a data service instance, which is on the interconnect. The data service instance sends out a totally-ordered message and waits to receive it, and returns the timestamp of said message to the query service. This timestamp will determine which version of a row the SQL statement in question will read until it is done running (this is not the case for the transaction as a whole, which is not point-in-time consistent by only read-committed). Write messages will be directed at every data service instance that supports a table. Each one will enqueue the write, append the current row to the row history, and update the row. These updates will be consistent across data service instances because write messages are totally ordered. If a row already has an uncommitted write then the data service will wait to return control to the client until the previous transaction has completed and it has applied the write. This is the behavior required for multi-version 2-phase locking, the behavior that applications expect.

When the transaction commits, the API service sends out a totally ordered commit message to all data services (this can be delegated to the data service, which is on the interconnect). When a data service instance receives the message it prunes the row history for its rows and releases the transaction's locks. A two-phase commit is not necessary because the locks on the rows in each data service instance were totally ordered and they were all previously granted. Safe delivery of transaction commits (or rollbacks) is not necessary either. If a processor fails it rolls back its in-flight transactions. When it merges with the cluster again it synchs up its state with the cluster and does not begin processing transactions until it is in synch.

# DONE

- Write some good "user data" to provision an ec2 instance for. It needs rust, vscode, and https access.
    - This is done. See user_data file in this repo. You will need to edit it and put your own key in it.

# TODO

- Figure out how to publish API documentation on github
- Write requirements for authentication
- Write requirements for backup
- Write requirements for metrics collection
- Write requirements for statistics collection

# Modules

In this section I describe some code modules or crates that we have identified so far.

- Totem single-ring protocol library
- Totem multiple-ring protocol library
- Rust driver
- Java driver
- Typescript driver
- C driver
- API service
- SQL compiler
- Query optimizer
- Query runner service
- Statistics collection library
- System data service
- Row-oriented, in-memory, replicated data service
- Column-oriented, in-memory, replicated data service
- B-tree index

# Miscellaneous notes

- How to set up EC2 development instance:
    - Use m4 class
    - Install rust as ec2-user
    - Install gcc as root (for the linker)
    - Install git as root
    - Scp ssh keys (they are used for github communication)
    - Install rust-analyzer vscode extension (you have to do re-do this each time)
    - When rust-analyzer breaks click on the icon at the bottom, stop and start the rust-analyzer server

- The query statistics service (QSS) needs to collect stats on every SQL query and sub-query used. This
is the only way to ensure that we use all the knowledge available about the contents of the database
short of running special queries to get it. For example, a SELECT * FROM mytable needs to collect statistics
on every column in the select list. These statistics will only apply to mytable and to this particular query.
Statement SELECT mycolumn FROM mytable can use the same statistics. Statement SELECT * FROM mytable WHERE
mycolumn = 1 will have its own statistics. So will SELECT * FROM mytable WHERE mycolumn = 2. None of this
information can be thrown away a priori. However, storing this information costs money, so the database
needs to maintain metrics on its statistics database and manage it. It needs to keep track of which statistics
are proving useful. And it has to present this information to the administrator and suggest changes (such as deleting certain data, or stopping collection of certain statistics), leaving the choice to the administrator. And it needs to keep track of which statements use which directives, so the administrator understands cause and effect in query performance. The statistics will benefit from a good compression algorithm because they will consume a lot of space.

# Backlog

- Write a driver that runs simple SQL statements (no parameters.)
- Write a listener service that receives the SQL from driver.
- Use TCP for the protocol.
- The listener calls the parser, which parses the SQL statement into an AST.
- The listener calls the type checker, which checks the AST and produces an abstract query plan.
- The listener calls the optimizer, which turns the abstract query plan into a real query plan.
- The optimizer calls the query statistics service (QSS) to find out stats about the objects.
- The listener calls the table representation service (TRS) in the plan and runs the statement.
- The driver requests a session ID. Figure out a way for the listener to generate this.
- The type checker calls the data dictionary service (DDS), to look up tables etc.
- The type checker gets a shared dictionary lock on the relevant items (from the DDS.)
- The TRS runs the query operation.
- The listener collects statistics and updates the QSS. 
- Add support for these SQL statements, in order:
    - A trivial INSERT statement.
    - A trivial UPDATE statement.
    - A trivial SELECT statement (full table scan).
    - COMMIT statement.
- Have two representations of a table.
- At this point you have all the main parts of the system: driver, listener, parser, data dictionary,
  optimizer, concurrency control, and replication.

# Add to Backlog

- Connection pooling. Must re-open broken streams to use old connections.
- Prepared statements.
- Offer HTTP and HTTPS as a choice of transport protocol.