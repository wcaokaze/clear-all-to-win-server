
起動
--------------------------------------------------------------------------------
```sh
docker run --env DATABASE_URL=<Postrges URL> -p 8000:8000 --rm -it clear-all-to-win-server
```

Postgresに接続するために `--net=host` が必要な場合はそれもつけてください

まあ大抵PostgresもDockerで起動するでしょうしNginxも必要ですから
docker-composeを用意した方がいいとは思います


認証
--------------------------------------------------------------------------------

正しいクライアントから送信されたリクエストであることと、
リクエストが通信経路で改ざんされていないことを保証するために
下記の認証を必須とする。


### API接続情報をURI化する

1. 接続先のAPIのスキーム、ドメイン、パスを用意する  
    `https://example.com/api/v1/gamerecords`
2. 送信するパラメータのうち、パスに含まれていないものを用意する  
    `/api/v1/gamerecords/:id` はパス部分に `id` パラメータをとるのでこれは不要。  
    パラメータがJSONなどで、配列やネストされた値を持つ場合、すべて `[]` で展開する。  
    ```json
    {
       "a": [0, 1, 1, 2, 3, 5],
       "b": {
          "c": 8,
          "d": [13, 21]
       }
    }
    ```
    ↓  
    `a[0]=1` `a[1]=1` `a[2]=2` `a[3]=3` `a[4]=5` `b[c]=8` `b[d][0]=13` `b[d][1]=21`
3. さらに下記パラメータを用意する  
    `nonce` ランダム生成した文字列  
    `timestamp` API接続を試みた時刻(yyyyMMdd'T'HHmm'Z')  
3. 2, 3をキー名の昇順にソートしてUTF-8でパーセントエンコードし、1の後にクエリとして連結する。
    `https://example.com/api/v1/gamerecords?a%5B0%5D=1&nonce=r7rWYAHMR3psD7aNWoYsxVyKAkYbed1g&timestamp=20200623T1217Z`

### signatureを生成する

上記手順で生成したURIをHMAC-SHA256でハッシュ化したものを `signature` とする。
鍵はあらかじめ発行済のトークンを使う。

### HTTPリクエストヘッダーに付与する

`nonce`, `timestamp`, `signature`をカンマ区切りで付与する。

`Authorization: catw nonce=r7rWYAHMR3psD7aNWoYsxVyKAkYbed1g,timestamp=20200623T1217Z,signature=xxxxxxxx`


API
--------------------------------------------------------------------------------

### POST `/api/v1/gamerecords`

棋譜を保存する。

##### パラメータ

GameRecordのJSON。 `id` は指定不要。
Rocketの仕様次第ではJSON以外も受け付けるかもしれないけどそれはたまたまってことで

##### レスポンス

- 200  
    保存されたGameRecordのJSON
- 400  
    指定された初期フィールドとルールに従って操作の再現をした最終結果のフィールドに
    trueが混ざっている場合
- 406  
    `Accept` リクエストヘッダーでJSONを受け付けなかった場合


### GET `/api/v1/gamerecords/:id`

棋譜を取得する。

##### パラメータ

`id`: ID

##### レスポンス

- 200  
    GameRecordのJSON。
- 404  
    指定したIDの棋譜が存在しない場合
- 406  
    `Accept` リクエストヘッダーでJSONを受け付けなかった場合


エンティティ
--------------------------------------------------------------------------------

## GameRecord

- `id`: String  
    ID。POST時省略可。
- `player_name`: String?  
    プレイヤー名(null可)
- `start_time`: String  
    ゲーム開始時刻(yyyyMMdd'T'HHmm'Z')
- `initial_field`: Field  
    ゲーム開始直後のフィールド。
- `rule`: [[Bool]]  
    ルール。3×3の二次元配列。フィールドの押された場所を中心として周囲1マスのセルに
    この二次元配列とのXOR演算を行う。わかりやすい言い方をすれば
    ルールでtrueになっているセルが反転する。
- `steps`: [Step]  
    操作した手順。

## Field

- `width`: Int  
    幅
- `height`: Int  
    高さ
- `cells`: [[Bool]]  
    セルの状態。セルの配列で行を表し、行の配列でフィールドを表す。
    つまりアクセス時 `cells[x][y]` の形になる。
    width, heightと一致しないサイズの配列は不正値とする。

## Step

- `time`: Int  
    ゲーム開始からの経過時間(ミリ秒)
- `point`: [Int]  
    押した場所

```json
{
   "id": "1592850533822",
   "player_name": "wcaokaze",
   "start_time": 1592850520968,
   "initial_field": {
      "width": 5,
      "height": 5,
      "cells": [
         [true, true, false, true, true],
         [true, false, true, false, true],
         [false, true, true, true, false],
         [true, false, true, false, false],
         [true, true, false, false, false]
      ]
   },
   "rule": [
      [false, true, false],
      [true, true, true],
      [false, true, false]
   ],
   "steps": [
      {
         "time": 0,
         "point": [3, 2]
      },
      {
         "time": 1000,
         "point": [3, 2]
      }
   ]
}
```


