# API design

POST - /register

- user_id: String
- password: String

- group_id: String
- token: String
- refresh_token: String

POST - /login

- user_id: String
- password: String

- group_id: String
- token: String
- refresh_token: String

GET - /group/{:group_id}
PUT - /group/{:group_id}
DELETE - /group{:group_id}

POST - /{:group_id}/register
POST - /{:group_id}/loging
DELETE - /{:group_id}/{:user_id}
