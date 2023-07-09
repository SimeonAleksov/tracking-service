## Installing


```shell
$ docker compose up
```

Insert few rows:
```postgresql
insert into account (first_name, last_name, is_active)
values ('Simeon', 'Aleksov', true);
```

Make few requests

```shell
curl --location --request POST 'http://localhost:8000/tracking/38aded55-23cc-4f38-bb64-70e906a11e5d/'
```

 or JavaScript
 
```javascript
var raw = "";

var requestOptions = {
  method: 'POST',
  body: raw,
  redirect: 'follow'
};

fetch("http://localhost:8000/tracking/38aded55-23cc-4f38-bb64-70e906a11e5d/", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));m
```