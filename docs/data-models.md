# ARGUS Data Models

## AircraftState

Represents a single aircraft telemetry record.

Fields:

| Field          | Type     | Description                |
| -------------- | -------- | -------------------------- |
| icao24         | string   | unique aircraft identifier |
| callsign       | string   | flight identifier          |
| origin_country | string   | country of origin          |
| latitude       | float    | aircraft latitude          |
| longitude      | float    | aircraft longitude         |
| altitude       | float    | aircraft altitude          |
| velocity       | float    | aircraft speed             |
| timestamp      | datetime | record creation time       |

---

## Database Table

Table: aircraft_states

Schema:

```
CREATE TABLE aircraft_states (
    id SERIAL PRIMARY KEY,
    icao24 TEXT,
    callsign TEXT,
    origin_country TEXT,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    altitude DOUBLE PRECISION,
    velocity DOUBLE PRECISION,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

---

## Future Data Models

Future versions of ARGUS may include:

DroneState

SensorEvent

WeatherObservation

ThreatEvent
