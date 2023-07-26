double-dict-server
==================

A very simple double-dict storage.

Runs on 0.0.0.0:9000

endpoints
=========
/api/health: GET, returns text "OK"
/api/version: GET, returns text with the current version
/api/post/one/PRIMARY\_KEY/SECONDARY\_KEY: POST, store data (request body)
/api/get/one/PRIMARY\_KEY/SECONDARY\_KEY: GET, read data
/api/get/all/PRIMARY\_KEY: GET, read all data associated with that primary key (messagepack-encoded dictionary)
/api/delete/one/PRIMARY\_KEY/SECONDARY\_KEY: DELETE, remove data (will remove PRIMARY\_KEY if SECONDARY\_KEY was the last key in that dictionary)
/api/delete/all/PRIMARY\_KEY: DELETE, remove the PRIMARY\_KEY
