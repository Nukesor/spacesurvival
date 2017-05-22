module Api.Research exposing (..)

import Api.Util exposing (..)
import Http exposing (emptyBody, expectJson)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages
import Model.Research exposing (..)
import Model.User exposing (User(LoggedIn))
import Dict


researchesDecoder : Decode.Decoder (Dict.Dict String Research)
researchesDecoder =
    dataDecoder <|
        Decode.dict <|
            Decode.succeed Research
                |: (Decode.field "name" Decode.string)
                |: (Decode.field "current_level" (Decode.maybe Decode.int))
                |: (Decode.field "dependencies" <|
                        Decode.oneOf
                            [ Decode.list <|
                                Decode.map2 (,)
                                    (Decode.index 0 Decode.string)
                                    (Decode.index 1 Decode.int)
                            , Decode.null []
                            ]
                   )
                |: (Decode.field "levels" (Decode.list researchLevelDecoder))


researchLevelDecoder : Decode.Decoder ResearchLevel
researchLevelDecoder =
    Decode.succeed ResearchLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "resources" <|
                Decode.list <|
                    Decode.map2 (,)
                        (Decode.index 0 Decode.string)
                        (Decode.index 1 Decode.int)
           )


fetchResearches : { a | user : User } -> Cmd Messages.Msg
fetchResearches model =
    case model.user of
        LoggedIn user ->
            let
                request =
                    Http.request
                        { method = "GET"
                        , headers =
                            [ Http.header "Authorization" user.token
                            ]
                        , url = "/api/researches/pod"
                        , expect = expectJson researchesDecoder
                        , body = emptyBody
                        , timeout = Nothing
                        , withCredentials = False
                        }
            in
                Http.send Messages.ReceiveResearches request

        _ ->
            Cmd.none
