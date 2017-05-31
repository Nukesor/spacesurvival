module Api.Util exposing (..)

import Http exposing (emptyBody, expectJson, jsonBody, stringBody)
import Json.Decode as Decode
import Model exposing (Model)
import Model.User exposing (User(LoggedIn))


dataDecoder : Decode.Decoder a -> Decode.Decoder a
dataDecoder =
    Decode.field "data"


authenticatedGet :
    Model
    -> String
    -> Decode.Decoder a
    -> (Result Http.Error a -> msg)
    -> Cmd msg
authenticatedGet model url decoder msg =
    case model.user of
        LoggedIn user ->
            let
                request =
                    Http.request
                        { method = "GET"
                        , headers =
                            [ Http.header "Authorization" user.token
                            ]
                        , url = url
                        , expect = expectJson decoder
                        , body = emptyBody
                        , timeout = Nothing
                        , withCredentials = False
                        }
            in
                Http.send msg request

        _ ->
            Debug.log "Unable to send authorized request, no token!" Cmd.none


authenticatedPost model url decoder msg data =
    case model.user of
        LoggedIn user ->
            let
                request =
                    Http.request
                        { method = "POST"
                        , headers =
                            [ Http.header "Authorization" user.token
                            ]
                        , url = url
                        , expect = expectJson decoder
                        , body = jsonBody data
                        , timeout = Nothing
                        , withCredentials = False
                        }
            in
                Http.send msg request

        _ ->
            Debug.log "Unable to send authorized request, no token!" Cmd.none
