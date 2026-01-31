# Vanguard
Welcome to Vanguard's official Github Page

## Roadmap & Status
| ⭐ Feature                | ℹ️ Explanation                                                                                       | 🚀 Status      |
|--------------------------|------------------------------------------------------------------------------------------------------|----------------|
| DNS Integration          | Allows Vanguard to access DNS records and navigate following request to your standalone app           | ✅ Done        |
| HTTP Routing             | Fully implementation for HTTP navigation by domain - app endpoint                                     | ✅ Done        |
| SSL/TLS Support          | Ability for parsing SSL certificates and providing TLS handshake                                      | ✅ Done        |
| HTTPS Support            | Fully implementation for HTTPS navigation by domain, app endpoint, and SSL cert & private key         | ✅ Done        |
| Dynamic Data & Engine Config Support   | A runtime config path implementation. Works in Vanguard's lifecycle. Has observers to local storage upgrade on any real-time changes | ✅ Done  |
| Integrated Web Server    | An Integrated web server provides ability to serve a directory with its all contents without any external web server solution. Also allows domain & DNS bindings | ✅ Done  |
| Warning & Error Logging  | A feature which responsible of realtime logging of Vanguard system errors & warnings. Logs can be viewed on filesystem or CLI |  ✅ Done |
| CLI Application   | A shell (CLI) application for controlling Vanguard Engine (server) realtime | ✅ Done  |
| Keep-Alive Connection Support   | Keep-Alive connection type implemented to IWS&Secure IWS engines so clients doesn't need to  reconnect for every asset (css, js, images)  | ✅ Done  |
| Zero-Copy  Support  | Instead of reading files with buffer, transfering them directly to the network socket without memory allocation | ✅ Done  |
| Enhanced Scalability  |  Implement scalability measures for handling large number of requests. Including http timeouts, server read and maximum connection limitations etc. | ⌛ In Progress  |
|  Rate Limiting | Very specific configurable rate limiting mechanism to prevent abuse and protect backend services & Vanguard's itself.| ⌛ In Progress  |
| Pattern Based Routing & Upstream HTTPS support  | Allowing routings with patterns like *.example.com etc.| ⚠️ Not Started Yet |
| GUI Application   | A web based control panel (GUI application) for controlling Vanguard Engine (server)  realtime  | ⚠️ Not Started Yet |
| Live Metrics Implementation  | Tracking real-time metrics that allows administration to track request rates, latencies, frequencies| ⚠️ Not Started Yet |
| Load Balancer Implementation  | Implementing load balance mechanism to Vanguard engine, with various load balancing  algorithms| ⚠️ Not Started Yet |
|  Health Checks |Health check mechanism allow Vanguard's load balancer to decide that ensure backend apps are available before routing traffic to them.| ⚠️ Not Started Yet |
|   IP Blocking | IP based client blocking mechanism to prevent abuse and protect backend services & Vanguard's itself.| ⚠️ Not Started Yet |
| Buffer Tuning  | Adjusting buffer sizes for read/write streams can optimize throughput | ⚠️ Not Started Yet  |
| Real-Time Logging & Monitoring  | A feature allows user to track real time logs fromf Vanguard system for monitoring errors & warnings. | ⚠️ Not Started Yet |
