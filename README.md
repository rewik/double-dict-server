double-dict-server
==================

A very simple double-dict storage.

Runs on 0.0.0.0:9000

endpoints
=========
* ```/api/health```: GET, returns text "OK"
* ```/api/version```: GET, returns text with the current version
* ```/api/post/one/PRIMARY_KEY/SECONDARY_KEY```: POST, store data (request body)
* ```/api/get/one/PRIMARY_KEY/SECONDARY_KEY```: GET, read data
* ```/api/get/all/PRIMARY_KEY```: GET, read all data associated with that primary key (messagepack-encoded dictionary)
* ```/api/delete/one/PRIMARY_KEY/SECONDARY_KEY```: DELETE, remove data (will remove ```PRIMARY_KEY``` if ```SECONDARY_KEY``` was the last key in that dictionary)
* ```/api/delete/all/PRIMARY_KEY```: DELETE, remove the ```PRIMARY_KEY```
