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
