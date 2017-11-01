port module Api.Auth exposing (..)

import Json.Encode as Encode
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Model exposing (..)
import Model.User exposing (..)
import Http
import Messages


port readToken : () -> Cmd msg


port saveToken : LoggedInData -> Cmd msg


port receiveToken : (LoggedInData -> msg) -> Sub msg


registerEncoder : RegisterData -> Encode.Value
registerEncoder user =
    Encode.object
        [ ( "nickname", Encode.string user.nickname )
        , ( "email", Encode.string user.email )
        , ( "password", Encode.string user.password )
        ]


registerDecoder : Decode.Decoder LoginData
registerDecoder =
    Decode.map2 registeredUser (Decode.field "nickname" Decode.string) (Decode.field "email" Decode.string)


loginEncoder : LoginData -> Encode.Value
loginEncoder user =
    Encode.object [ ( "identifier", Encode.string user.identifier ), ( "password", Encode.string user.password ) ]


loginDecoder : Decode.Decoder LoggedInData
loginDecoder =
    Decode.succeed LoggedInData
        |: (Decode.field "current_auth_token" Decode.string)
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "pod" Decode.string)


registeredUser : String -> String -> LoginData
registeredUser nickname email =
    { identifier = nickname
    , password = ""
    }


register : Model -> Cmd Messages.Msg
register model =
    case model.user of
        Registering user ->
            let
                request =
                    Http.post "/api/user/register" (Http.jsonBody <| registerEncoder user) registerDecoder
            in
                Http.send Messages.Registered request

        _ ->
            Cmd.none


login : Model -> Cmd Messages.Msg
login model =
    case model.user of
        LoggingIn user ->
            let
                request =
                    Http.post "/api/auth/login" (Http.jsonBody <| loginEncoder user) loginDecoder
            in
                Http.send Messages.LoggedIn request

        _ ->
            Cmd.none
