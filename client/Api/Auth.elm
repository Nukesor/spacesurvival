module Api.Auth exposing (..)

import Json.Encode as Encode
import Json.Decode as Decode
import Model exposing (..)
import Model.User exposing (..)
import Http
import Messages


userEncoder : User -> Encode.Value
userEncoder user =
    Encode.object
        [ ( "nickname", Encode.string user.nickname )
        , ( "email", Encode.string user.email )
        , ( "password", Encode.string user.password )
        ]


registerDecoder =
    Decode.field "data" (Decode.map2 createUser (Decode.field "nickname" Decode.string) (Decode.field "email" Decode.string))


createUser nickname email =
    { token = Nothing
    , nickname = nickname
    , email = email
    , password = ""
    }


register : Model -> Cmd Messages.Msg
register model =
    let
        request =
            Http.post "/api/user/register" (Http.jsonBody <| userEncoder model.user) registerDecoder
    in
        Http.send Messages.Registered request
