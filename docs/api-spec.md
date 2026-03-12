# ARGUS API Specification

## Overview

The ARGUS API provides access to stored aircraft telemetry and analytical results.

The API will be implemented using **FastAPI**.

---

## Endpoints

### Get Aircraft

```
GET /aircraft
```

Returns recent aircraft telemetry.

Response:

```
[
  {
    "callsign": "DAL1121",
    "latitude": 33.64,
    "longitude": -84.43,
    "altitude": 8200
  }
]
```

---

### Get Aircraft History

```
GET /aircraft/{icao24}/history
```

Returns historical trajectory data for a specific aircraft.

---

### Get Active Aircraft Count

```
GET /aircraft/count
```

Returns the number of aircraft currently tracked.

---

## Authentication

Future versions will support:

* API keys
* JWT authentication
