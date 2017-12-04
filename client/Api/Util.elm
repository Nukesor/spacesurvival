module Api.Util exposing (..)

import Http exposing (emptyBody, expectJson, jsonBody, stringBody)
import Json.Decode as Decode
import Model exposing (Model)
import Model.User exposing (User(LoggedIn))
import Json.Encode
import Time.DateTime exposing (DateTime, fromISO8601)
import Time.TimeZones
import Time.ZonedDateTime exposing (toDateTime)


dateDecoder : Decode.Decoder DateTime
dateDecoder =
    Decode.map
        (Time.ZonedDateTime.fromISO8601 (Time.TimeZones.utc ())
            >> unwrap
            >> toDateTime
        )
        Decode.string


pairDecoder : Decode.Decoder a -> Decode.Decoder b -> Decode.Decoder ( a, b )
pairDecoder a b =
    Decode.map2 (,) (Decode.index 0 a) (Decode.index 1 b)


podUrl user suffix =
    case user of
        LoggedIn user ->
            "/api/pod/" ++ user.podId ++ suffix

        _ ->
            ""


unwrap : Result String b -> b
unwrap res =
    case res of
        Ok val ->
            val

        Err err ->
            Debug.crash err


createBody maybeData =
    case maybeData of
        Just data ->
            jsonBody data

        Nothing ->
            jsonBody Json.Encode.null


createRequest :
    Model
    -> String
    -> String
    -> Decode.Decoder a
    -> Maybe Json.Encode.Value
    -> (Result Http.Error a -> msg)
    -> Cmd msg
createRequest model method url decoder maybeData msg =
    case model.user of
        LoggedIn user ->
            Http.send
                msg
                (Http.request
                    { method = method
                    , headers =
                        [ Http.header "Authorization" (user.id ++ ":" ++ user.token)
                        ]
                    , url = url
                    , expect = expectJson decoder
                    , body = createBody maybeData
                    , timeout = Nothing
                    , withCredentials = False
                    }
                )

        _ ->
            Debug.log "Unable to send authorized request, no token!" Cmd.none


authenticatedGet model url decoder msg =
    createRequest model "GET" url decoder Nothing msg


authenticatedPost model url decoder msg data =
    createRequest model "POST" url decoder (Just data) msg
