
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


# Device Lookup
API to detail and lookup local network IPs and devices.

Author: MPZinke
Created on: 2022.04.28


## Description
As a Home Automator,
I want a system that organizes my attached devices, and provides a way to access them by IP, Type, current connection, and network,
So that I can easily and dynamically gather information and expand my network and automations.


### Additional Features
- Currently connected devices
- Hosts' names sugar (use the device.label as a way of getting device info)


### Desired Implementations
- Who Is Home
- Smart Curtain


### Behavior
A HTTP request will access the REST API, which will read from the Postgres DB.
If the request is for device specific info (information containing device address specifics) then the router will make an HTTP get request to the router and parse its HTML to see information like whether the address currently has a device connected to it (if no DB values exists for it).
Additionally, the ping method can be used to determine if a device is active/responsive on an address.


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
- `/api/v1.0/network/id/{id}/device`: "Queries for device based on network id"
- `/api/v1.0/network/id/{id}/device/address`: "Queries for device based on device address and network id"
- `/api/v1.0/network/id/{id}/device/address/{address}`: "Get a device by device address and network id"
- `/api/v1.0/network/id/{id}/device/id`: "Queries for device based on device id and network id"
- `/api/v1.0/network/id/{id}/device/id/{id}`: "Get a device by device id and network id"
- `/api/v1.0/network/id/{id}/device/label`: "Queries for device based on device label and network id"
- `/api/v1.0/network/id/{id}/device/label/{label}`: "Get a device by device label and network id"
- `/api/v1.0/network/id/{id}/devices`: "Queries for devices based one network id"
- `/api/v1.0/network/id/{id}/devices/all`: "List all devices for network id"
- `/api/v1.0/network/id/{id}/devices/group`: "Queries for devices based on group and network id"
- `/api/v1.0/network/id/{id}/devices/group/id`: "Queries for devices based on group id and network id"
- `/api/v1.0/network/id/{id}/devices/group/id/{id}`: "List all devices based on group id and network id"
- `/api/v1.0/network/id/{id}/devices/group/label`: "Queries for devices based on group label and network id"
- `/api/v1.0/network/id/{id}/devices/group/label/{label}`: "List all devices based on group label and network id"
- `/api/v1.0/network/label`: "Queries for a network based on network label"
- `/api/v1.0/network/label/{label}`: "Get a network by label"
- `/api/v1.0/network/label/{label}/device`: "Queries for device based on network label"
- `/api/v1.0/network/label/{label}/device/address`: "Queries for device based on device address and network label"
- `/api/v1.0/network/lable/{label}/device/address/{address}`: "Get a device by device address and network label"
- `/api/v1.0/network/label/{label}/device/id`: "Queries for device based on device id and network label"
- `/api/v1.0/network/label/{label}/device/id/{id}`: "Get a device by device id and network label"
- `/api/v1.0/network/label/{label}/device/label`: "Queries for device based on device label and network label"
- `/api/v1.0/network/label/{label}/device/label/{label}`: "Get a device by device label and network label"
- `/api/v1.0/network/label/{label}/devices`: "Queries for devices based one network label"
- `/api/v1.0/network/label/{label}/devices/all`: "List all devices for network label"
- `/api/v1.0/network/label/{label}/devices/group`: "Queries for devices based on group and network label"
- `/api/v1.0/network/label/{label}/devices/group/id`: "Queries for devices based on group id and network label"
- `/api/v1.0/network/label/{label}/devices/group/id/{id}`: "List all devices based on group id and network label"
- `/api/v1.0/network/label/{label}/devices/group/label`: "Queries for devices based on group label and network label"
- `/api/v1.0/network/label/{label}/devices/group/label/{label}`: "List all devices based on group label and network label"


## Compiling
```bash
export NETWORKLOOKUP_BEARERTOKEN=""
export NETWORKLOOKUP_ROUTER_DOMAIN=""

cargo build
```



## Appendix

- Router Attached Devices URL: `http://<ROUTER GATEWAY>/DEV_device_dev_id.htm`
