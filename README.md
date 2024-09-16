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

There will be a virtual synchrony service. The virtual synchrony service is highly available and fault tolerant. If a node fails in the virtual synchrony service no data is lost. The purpose of the virtual synchrony service is to maintain a consistent view of progress throughout the system so that when a node fails any transactions that ought to be rolled back are rolled back. Every process in the system connects to the virtual synchrony service and acquires a node id. Each node sends heartbeats to the virtual synchrony service, so that if the node fails the virtual synchrony service can take action to recover. Nodes communicate with each other directly, point to point. The virtual synchrony service provides the way for nodes interested in the same topic to find each other. Messages are spread over several topics. Progress is tracked using sequential ids and high water marks. When a node sends a heartbeat to the virtual synchrony service it sends the high water marks for the topics to which it is subscribed. The existence of the virtual synchrony service does not imply implementation details for how nodes communicate. The details can change depending on the purpose of a topic. Some replication protocols are designed for small groups, whereas others are appropriate for large groups.

# How Services Work Together

In this section I will discuss how the aforementioned architecture enables users to get work done in the database.

An application loads the driver software with the connection string. The connection string contains a DNS record that returns one of a list of IP address, corresponding to the API service.

The driver calls API service and requests a session ID. The API service has previously registered with the virtual synchrony service and been assigned a node id. It creates a session ID consisting of its node id and a session sequence number which reset to zero when the last view was installed (in the sense of virtual synchrony). It returns the session ID to the client.

The driver calls the API service and requests a transaction id. The API service generates a transaction id consisting of its (aforementioned) node id and a sequence number. It sends this transaction ID to the transaction lifecycle topic, informing members that the transaction has started. It then returns the transaction ID to the client.

The driver calls the API service and sends it its session id and transaction id, and a SQL statement. The API service calls the query planner and receives a plan for the query. It then calls the query runner service.

The query runner service processes the next step in the query plan. The processing of query steps will be described elsewhere. When the query runner needs to read or write data, it sends a message to a data service. Read messages behave differently than write messages. Read messages are directed at a specific data service instance which is specified in the query plan. For example, one plan may scan a column-oriented store, while another a row-oriented store. When the data service instance scans a row it uses the row version consistent with the transactions's system change number. Write messages will be directed at every data service instance that supports a table. Each one will enqueue the write, eventually grant the lock, append the current row to the row history, and update the row. These updates will be consistent across data service instances because the messages in each topic are totally ordered.

When the transaction commits, the API service sends a commit message to the transaction lifecycle topic. All data service instances subscribe to this topic. When a data service instance receives the message it prunes the row history for its rows and releases the transaction's locks. A two-phase commit is not necessary because the locks on the rows in each data service instance were totally ordered (on the table topic) and they were all previously granted.

When a node fails the virtual synchrony service detects this and notifies all nodes. The nodes pause further actions and run recovery. All missed messages that can be retransmitted are retransmitted. Where necessary, transactions are rolled back. After all nodes have run their recovery the virtual synchrony service installs a new view of the cluster and processing continues.

# TODO

- Authentication