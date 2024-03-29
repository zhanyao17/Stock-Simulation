# Stock-Simulation
Using rust to run a simulation of stock trading system 


# Documentation
Languages
- Safety issues in rust (error handling)

- RabbitMQ (AMQP)
    - Rabbitmq vs kafka
    - Latency: RabbitMQ can introduce some latency due to message queuing and routing processes. For applications requiring ultra-low latency, such as high-frequency trading or real-time gaming, RabbitMQ might not be the best choice. In such cases, you might need a messaging solution optimized for low latency, like ZeroMQ or Redis Pub/Sub.
    - Throughput: RabbitMQ is capable of handling high message throughput, especially when configured with multiple nodes in a cluster. However, achieving high throughput requires proper configuration and optimization. For extremely high-throughput real-time systems, specialized solutions might be required.
    - Message Ordering: RabbitMQ guarantees message ordering within a single queue. However, if your application requires strict global message ordering across multiple queues or topics, achieving this in RabbitMQ can be challenging and might require additional synchronization mechanisms.
    - Scalability: RabbitMQ can scale horizontally by setting up clusters with multiple nodes. This allows you to distribute message processing across multiple servers for increased throughput and fault tolerance. However, managing a RabbitMQ cluster introduces additional complexity compared to standalone deployments.
    - Reliability: RabbitMQ provides features like durable queues, message acknowledgments, and persistent message storage to ensure reliable message delivery. This makes it suitable for many real-time applications where message reliability is critical.
    - Integration: RabbitMQ integrates well with various programming languages and platforms, making it suitable for building real-time systems with diverse technology stacks. It supports many protocols and client libraries, facilitating integration with existing applications and services.
- amiquip
- benchmarks
- receive witha specific time duration


# Todo:
- Infinity loop in stocks
- stock price randomly decrease we got too many stocks, cut loss had to try [Check whether nid decrease every time] // create a var to triggered this

