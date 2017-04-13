module Api.Auth exposing (..)

import Json.Encode as Encode
import Json.Decode as Decode
import Model exposing (..)
import Model.User exposing (..)
import Http
import Messages


dataDecoder : Decode.Decoder a -> Decode.Decoder a
dataDecoder =
    Decode.field "data"


registerEncoder : User -> Encode.Value
registerEncoder user =
    case user of
        Registering user ->
            Encode.object
                [ ( "nickname", Encode.string user.nickname )
                , ( "email", Encode.string user.email )
                , ( "password", Encode.string user.password )
                ]

        _ ->
            Encode.null


registerDecoder : Decode.Decoder LoginData
registerDecoder =
    dataDecoder <| Decode.map2 registeredUser (Decode.field "nickname" Decode.string) (Decode.field "email" Decode.string)


loginEncoder : User -> Encode.Value
loginEncoder user =
    case user of
        LoggingIn user ->
            Encode.object [ ( "identifier", Encode.string user.identifier ), ( "password", Encode.string user.password ) ]

        _ ->
            Encode.null


loginDecoder : Decode.Decoder LoggedInData
loginDecoder =
    Decode.map loggedInUser (dataDecoder Decode.string)


registeredUser : String -> String -> LoginData
registeredUser nickname email =
    { identifier = nickname
    , password = ""
    }


loggedInUser : String -> LoggedInData
loggedInUser token =
    { token = token }


register : Model -> Cmd Messages.Msg
register model =
    case model.user of
        Registering user ->
            let
                request =
                    Http.post "/api/user/register" (Http.jsonBody <| registerEncoder model.user) registerDecoder
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
                    Http.post "/api/auth/login" (Http.jsonBody <| registerEncoder model.user) loginDecoder
            in
                Http.send Messages.LoggedIn request

        _ ->
            Cmd.none
