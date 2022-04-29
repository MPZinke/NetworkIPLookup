
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
- `/v1.0`
- `/v1.0/networks`
- `/v1.0/network/id/{network id}`
- `/v1.0/network/id/{network id}/ips/`
- `/v1.0/network/id/{network id}/ips/is_reservation/{ip is_reservation}`
- `/v1.0/network/id/{network id}/ips/is_static/{ip is_static}`
- `/v1.0/network/id/{network id}/ips/DeviceTypes/id/{DeviceTypes id}`: Get all IPs for a DeviceType
- `/v1.0/network/id/{network id}/ips/DeviceTypes/label/{DeviceTypes label}`: Get all IPs for a DeviceType
- `/v1.0/network/id/{network id}/ip/id/{ip id}`
- `/v1.0/network/id/{network id}/ip/label/{ip label}`
- `/v1.0/network/id/{network id}/ip/address/{ip address}`
- `/v1.0/network/label/{network label}`
- `/v1.0/network/label/{network label}/ips/`
- `/v1.0/network/label/{network label}/ips/is_reservation/{ip is_reservation}`
- `/v1.0/network/label/{network label}/ips/is_static/{ip is_static}`
- `/v1.0/network/label/{network label}/ips/DeviceTypes/id/{DeviceTypes id}`: Get all IPs for a DeviceType
- `/v1.0/network/label/{network label}/ips/DeviceTypes/label/{DeviceTypes label}`: Get all IPs for a DeviceType
- `/v1.0/network/label/{network label}/ip/id/{ip id}`
- `/v1.0/network/label/{network label}/ip/label/{ip label}`
- `/v1.0/network/label/{network label}/ip/address/{ip address}`
- `/v1.0/DeviceTypes`: Get all DeviceTypes
- `/v1.0/DeviceType/id/{DeviceTypes id}`: Get DeviceType info by id
- `/v1.0/DeviceType/label/{DeviceTypes label}`: Get DeviceType info by label



## Appendix

- Router Attached Devices URL: `http://<ROUTER GATEWAY>/DEV_device_dev_id.htm`
