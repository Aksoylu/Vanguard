# Vanguard CLI Commands
There are 21 different CLI commands that allow you to control Vanguard Engine which is running on your server. 

## 1.General Commands

### 1.1 Echo
The Echo command transmits a given string to the server, which then mirrors it back to the sender. This function serves to verify the connectivity between the Vanguard Engine and the CLI

__Usage:__
```
>>>  echo "hello"
```
__Result:__
```
[INFO] Echo answer from Vanguard Engine: hello
```

### 1.2 Status
Displays the current operational state of the Vanguard Engine, including server socket bindings, forwarding activity, base configurations and system health.

__Usage:__
```
>>>  status
```
__Result:__
```
Http & IWS Server        ● Running (Socket: 0.0.0.0:80) 
Http Forwarding          ► Forwarding (3) 
IWS Forwarding           ► Forwarding (1)
Https Server             ● Running (Socket: 0.0.0.0:443)
Https Forwarding         ⏸︎ Waiting (Idle)
Secure IWS Forwarding    ⏸︎ Waiting (Idle)
```

__Notes:__
- If you just want to open your specific directory to web  using http protocol, you don't need to build and run a standalone application. Internal web server allows you to directly serve your content
- IWS (Internal Web Server) is a plain web server which allow you to host directly a folder on a port.

### 1.3 Version

Displays version information for Vanguard CLI and Vanguard Engine, and checks for available updates from the remote repository.

__Usage:__
```
>>> version
```

__Result:__
```
Vanguard CLI
  Build: 1.2.3
  Version: stable-release

Vanguard Engine
  Build: 1.2.3 (stable-release)

Checking for updates ...
Latest Vanguard Version
  Build: 1.2.3 (stable-release)

✓ Your Vanguard version is up to date
```

__Example Output (Update Available):__
```
Vanguard CLI
  Build: 1.2.0
  Version: stable-release

Vanguard Engine
  Build: 1.2.0 (stable-release)

Checking for updates ...
Latest Vanguard Version
  Build: 1.2.3 (stable-release)

⚠ Your Vanguard version is outdated. We strongly suggest you to keep your Vanguard version up to date
You can update your Vanguard version by following instructions at: https://vanguard.example.com/update
```

__Notes:__

- Requires an active connection to the Vanguard Engine
- Requires internet access to check for remote updates
- Automatically compares local and remote versions
- Provides update instructions when a newer version is available


## 2. Engine & System
### 2.1 @todo@ Getting configurations of Vanguard Engine 
### 2.2 @todo@ Configuring Vanguard Engine
### 2.3 @todo@ Restarting Engine
### 2.4 @todo@ Shutting down Engine

## 3. Route Management

### 3.1 Getting route list

Displays the configured routes in the Vanguard Engine. You can view routes by type (HTTP, HTTPS, IWS, Secure IWS) or view all routes at once.

__Usage:__
```
>>> get-route-list <route_type>
```

__Parameters:__

- `route_type`: The type of routes to display
  - `http` - HTTP routes
  - `https` - HTTPS routes with SSL configuration
  - `iws` - Integrated Web Server routes
  - `secure-iws` - Secure Integrated Web Server routes
  - `all` - All route types

__Result:__
```
------------------------------------

--- HTTP Routes (2) ---
#1  Domain: example.com
  Target: http://localhost:3000

#2  Domain: api.example.com
  Target: http://localhost:8080

--- HTTPS Routes (1) ---
#1  Domain: secure.example.com
  Target: https://localhost:3443
  SSL Certificate path: /etc/ssl/certs/cert.pem
  SSL Private Key path: /etc/ssl/private/key.pem

--- Integrated Web Server (IWS) Routes (1) ---
#1  Domain: static.example.com
  Serving Path: /var/www/static

--- Secure Integrated Web Server (IWS) (1) ---
#1  Domain: secure-static.example.com
  Target: /var/www/secure
  SSL Certificate path: /etc/ssl/certs/secure-cert.pem
  SSL Private Key path: /etc/ssl/private/secure-key.pem
------------------------------------
```

__Examples:__
```
>>> get-route-list http
>>> get-route-list https
>>> get-route-list all
```

### 3.2 Adding new Http route

Adds a new HTTP route to the Vanguard Engine, mapping a source domain to a target URL for HTTP traffic forwarding.

__Usage:__
```
>>> add-http-route <source> <target>
```

__Parameters:__

- `source`: The source domain to match incoming requests (e.g., `example.com`)
- `target`: The target URL to forward requests to (e.g., `http://localhost:3000`)

__Result:__
```
✓ New http route added successfully
```

__Examples:__
```
>>> add-http-route example.com http://localhost:3000
>>> add-http-route api.example.com http://localhost:8080
>>> add-http-route subdomain.mysite.com http://192.168.1.100:5000
```

__Notes:__

- The source domain should not include the protocol (http://)
- The target URL must include the full protocol and address
- Route changes take effect immediately without requiring a restart

### 3.3 Deleting HTTP Route

Removes an existing HTTP route from the Vanguard Engine by its source domain.

__Usage:__
```
>>> delete-http-route <source>
```

__Parameters:__

- `source`: The source domain of the route to delete (e.g., `example.com`)

__Result:__
```
✓ Http route deleted successfully
```

__Examples:__
```
>>> delete-http-route example.com
>>> delete-http-route api.example.com
>>> delete-http-route subdomain.mysite.com
```

__Notes:__

- The source domain must match an existing HTTP route
- Route deletion takes effect immediately
- If the route doesn't exist, an error message will be displayed
- Use `routes http` to view all existing HTTP routes before deletion

### 3.4 @todo@ Adding new Https route
### 3.5 @todo@ Removing a Https route
### 3.6 @todo@ Adding a new IWS route
### 3.7 @todo@ Removing a IWS route
### 3.7 @todo@ Adding a new IWS route
### 3.8 @todo@ Removing a IWS route

## 4. SSL & TLS Management
@@todo@@ About SSL support of Vanguard
@@todo@@ Use cases of SSL with Vanguard

## 5. Load Balancing
### 5.1 @todo@ Creating new load balancing task
#### 5.1.1 @todo@ Supported Load Balancing Algorithms
### 5.2 @todo@ Listing current load balancing tasks
### 5.3 @todo@ Configuring a load balancing task
### 5.4 @todo@ Removing a load balancing task

## 6 Logging
### 6.1 @todo@ Getting logger configurations of Vanguard Engine 
### 6.2 @todo@ Setting logger configurations of Vanguard Engine 
### 6.3 @todo@ Tracking live logs
### 6.4 @todo@ Getting last N log records
### 6.5 @todo@ 

## 7. Terminal Utility
### 7.1 @todo@ Clearing terminal
### 7.2 @todo@ Exit from terminal


