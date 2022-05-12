
<!----------------------------------------------------------------------------------------------------------------------
-                                                                                                                      -
-   Created by: MPZinke                                                                                                -
-   on 2022.04.28                                                                                                      -
-                                                                                                                      -
-   DESCRIPTION:                                                                                                       -
-   BUGS:                                                                                                              -
-   FUTURE:                                                                                                            -
-                                                                                                                      -
----------------------------------------------------------------------------------------------------------------------->


# IP Lookup
API to detail and lookup local network IPs and devices.

Author: MPZinke
Created on: 2022.04.28


## Description
As a Home Automator,
I want a system that organizes my attached devices, and provides a way to access them by IP, Type, current connection, and network,
So that I can easily and dynamically gather information and expand my network and automations.


### Additional Features
- Currently connected devices
- Hosts' names sugar (use the IP.label as a way of getting IP info)


### Desired Implementations
- Who Is Home
- Smart Curtain


### Behavior
A HTTP request will access the REST API, which will read from the Postgres DB.
If the request is for IP specific info (information containing IP address specifics) then the router will make an HTTP get request to the router and parse its HTML to see information like whether the address currently has a device connected to it (if no DB values exists for it).
Additionally, the ping method can be used to determine if an device is active/responsive on an address.


#### Desired Endpoints
- `/api`
- `/api/v1.0`
- `/api/v1.0/group`: "Queries for groups"
- `/api/v1.0/group/all`: "List all groups"
- `/api/v1.0/group/id`: "Get a group by ID path"
- `/api/v1.0/group/id/{id}`: "Get a group by ID"
- `/api/v1.0/group/label`: "Queries for group based on label"
- `/api/v1.0/group/label/{label}`: "Get a group by label"
- `/api/v1.0/network`: "Queries for networks"
- `/api/v1.0/network/all`: "List all networks"
- `/api/v1.0/network/id`: "Queries for a network based on network id"
- `/api/v1.0/network/id/{id}`: "Get a network by id"
- `/api/v1.0/network/id/{id}/ip`: "Queries for IP based on network id"
- `/api/v1.0/network/id/{id}/ip/id`: "Queries for IP based on IP id and network id"
- `/api/v1.0/network/id/{id}/ip/id/{id}`: "Get an IP by IP id and network id"
- `/api/v1.0/network/id/{id}/ip/label`: "Queries for IP based on IP label and network id"
- `/api/v1.0/network/id/{id}/ip/label/{label}`: "Get an IP by IP label and network id"
- `/api/v1.0/network/id/{id}/ips`: "Queries for IPs based one network id"
- `/api/v1.0/network/id/{id}/ips/all`: "List all IPs for network id"
- `/api/v1.0/network/id/{id}/ips/group`: "Queries for IPs based on group and network id"
- `/api/v1.0/network/id/{id}/ips/group/id`: "Queries for IPs based on group id and network id"
- `/api/v1.0/network/id/{id}/ips/group/id/{id}`: "List all IPs based on group id and network id"
- `/api/v1.0/network/id/{id}/ips/group/label`: "Queries for IPs based on group label and network id"
- `/api/v1.0/network/id/{id}/ips/group/label/{label}`: "List all IPs based on group label and network id"
- `/api/v1.0/network/label`: "Queries for a network based on network label"
- `/api/v1.0/network/label/{label}`: "Get a network by label"
- `/api/v1.0/network/label/{label}/ip`: "Queries for IP based on network label"
- `/api/v1.0/network/label/{label}/ip/id`: "Queries for IP based on IP id and network label"
- `/api/v1.0/network/label/{label}/ip/id/{id}`: "Get an IP by IP id and network label"
- `/api/v1.0/network/label/{label}/ip/label`: "Queries for IP based on IP label and network label"
- `/api/v1.0/network/label/{label}/ip/label/{label}`: "Get an IP by IP label and network label"
- `/api/v1.0/network/label/{label}/ips`: "Queries for IPs based one network label"
- `/api/v1.0/network/label/{label}/ips/all`: "List all IPs for network label"
- `/api/v1.0/network/label/{label}/ips/group`: "Queries for IPs based on group and network label"
- `/api/v1.0/network/label/{label}/ips/group/id`: "Queries for IPs based on group id and network label"
- `/api/v1.0/network/label/{label}/ips/group/id/{id}`: "List all IPs based on group id and network label"
- `/api/v1.0/network/label/{label}/ips/group/label`: "Queries for IPs based on group label and network label"
- `/api/v1.0/network/label/{label}/ips/group/label/{label}`: "List all IPs based on group label and network label"


## Compiling
```bash
export NETWORKIPLOOKUP_BEARERTOKEN=""
export NETWORKIPLOOKUP_ROUTER_DOMAIN=""

cargo build
```



## Appendix

- Router Attached Devices URL: `http://<ROUTER GATEWAY>/DEV_device_dev_id.htm`
