``` mermaid
sequenceDiagram
    participant  user as user
    participant  service as service
    participant  ea_auth as ea_auth
    
    service->>ea_auth: Group登録
    ea_auth->>ea_auth: Group作成
    ea_auth->>service: pub key送信
    user->>service: user登録 or ログイン
    service->>ea_auth: user 登録 or 認証
    ea_auth->>service: jwt token
    service->>user: jwt token
    user->>service: 何かしらのリクエスト with jwt token
    service->>service: jwt tokenをpub keyで検証
    service->>user: 何かしらのレスポンス
```