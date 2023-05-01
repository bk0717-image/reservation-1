# core reservation service
- Feature Name: core-reservation-service
- Start Date: 2023/4/30

## Summary

a core service for reservation

## Motivation

we built a common template for reservation system.1) hotel/room booking; 2) parking lot booking; 3)etc;

## Service Interface
```protobuf
enum ReserveStatus {
    UNKNOWN = 0;
    PENDING = 1;
    CONFIRMED = 2;
    BLOCKED = 3;
}

enum ReserveChangeType {
    UNKNOWN = 0;
    CREATE = 1;
    UPDATE = 2;
    DELETE = 3;
}

message Reservation {
    string id = 1;
    string user_id = 2;
    ReserveStatus status = 3;
    // resourse
    string resourse_id = 4;
    google.protobuf.Timestamp start = 5;
    google.protobuf.Timestamp end = 6;
    string notes = 7;
}

message ReserveRequest {
    Reservation reservation = 1;
}

message ReserveResponse {
    Reservation reservation = 1;
}

message UpdateRequest {
    ReserveStatus status = 1;
    string notes = 2;
}

message UpdateResponse {
    Reservation reservation = 1;
}

message CancelRequest {
    string id = 1;
}

message CancelResponse {
    Reservation reservation = 1;
}

message ConfirmRequest {
    string id = 1;
}

message ConfirmResponse {
    Reservation reservation = 1;
}

message GetRequest {
    string id = 1;
}

message GetResponse {
    Reservation reservation = 1;
}


message QueryRequest {
    string resoure_id = 1;
    string user_id = 2;
    // status is to filter, default UNKNOWN
    ReserveStatus status = 3;
    google.protobuf.Timestamp start = 4;
    google.protobuf.Timestamp end = 5;
}

message ListenRequest{}

message ListenResponse {

}

service ReservationService {
    rpc reserve(ReserveRequest) returns (ReserveResponse);
    rpc confirm(ConfirmRequest) returns(ConfirmResponse);
    rpc update(UpdateRequest) returns (UpdateReponse);
    rpc cancel(CancelRequest) returns(CancelResponse);
    rpc get(GetRequest) returns(GetResponse);
    rpc query(QueryRequest) returns(stream Reservation);
    rpc listen(ListenRequest) returns(ListenResponse);
}
```

### database schema

we use postgres as our DBMS

```sql
CREATE SCHEMA rsvp;

CREATE TYPE rsvp.reservation_status AS ENUM(
    'unknown','pending','confirmed','bloked'
);

CREATE TYPE rsvp.reservation_update_type AS ENUM(
    'unknown','create','update','delete'
);

CREATE TABLE rsvp.reservations (
    id uuid NOT NULL default uuid_generate_v4(),
    user_id varchar(64) NOT NULL,
    status rsvp.reservation_status NOT NULL,


    resourse_id varchar(64) NOT NULL,
    timespan tstzrange NOT NULL,


    note text,


    CONSTRAINT reservations_pkey primary key(id),
    CONSTRAINT reservations_conflict EXCLUDE
    USING gist (resourse_id WITH =,timespan WITH &&),
);

CREATE INDEX reservations_resourse_id_idx ON rsvp.reservations (resourse_id);
CREATE INDEX reservation_user_id_idx ON rsvp.reservations (user_id);


-- IF user_id is null returns the reservations of rid at during time
-- IF reservation_id is null returns the reservations of user_id at during time
-- IF both are null returns all reservations at during time
-- IF both set returns the reservations fit the values
CREATE OR REPLACE FUNCTION rsvp.query(uid text,rid text,during tstzrange) RETURNS TABLE rsvp.reservations AS $$ $$ LANGUAGE plpgsql;


-- trigger for add/update/cancel a reservation
CREATE TABLE rsvp.reservation_change(
    id SERIAL NOT NULL,
    reservation_id uuid NOT NULL,
    op rsvp.reservation_update_type NOT NULL,
);

CREATE OR REPLACE FUNCTION trigger() returns TRIGGER AS 
$$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO rsvp.reservation_changes(reservation_id,op) VALUES(NEW.id,'create');

    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes(reservation_id,op) VALUES(NEW.id,'update');
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO rsvp.reservation_changes(reservation_id,op) VALUES(OLD.id,'delete');
    END IF;
    NOTIFY reservation_update;
    RETURN NULL;
END;
$$ 
LANGUAGE plpgsql;

```

## Guide-level explanation


TBD

## Reference-level explanation


TBD

## Drawbacks

N/A

## Rationale and alternatives

N/A

## Prior art

N/A

## Unresolved questions
- How to handle repeated reservation?
- If load is big,we may use a extrnal queue for recording changes.
- We haven't considered observability/deployment yet.
- query performance might be an issue -need to revisit the index and also consider using cache.

## Future possibilities

TBD