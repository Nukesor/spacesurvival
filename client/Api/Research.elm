module Api.Research exposing (..)

import Api.Util exposing (..)
import Dict
import Json.Decode as Decode exposing (value)
import Json.Decode.Extra exposing ((|:))
import Json.Encode
import Messages
import Model exposing (Model)
import Model.Research exposing (..)
import Model.User exposing (User(LoggedIn))


researchesDecoder : Decode.Decoder Researches
researchesDecoder =
    Decode.dict <|
        Decode.succeed Research
            |: (Decode.field "type" Decode.string)
            |: (Decode.field "display_name" Decode.string)
            |: (Decode.succeed Nothing)
            |: Json.Decode.Extra.withDefault [] (Decode.field "dependencies" dependencyList)
            |: (Decode.field "levels" (Decode.list researchLevelDecoder))


dependencyList : Decode.Decoder (List ( ResourceId, Int ))
dependencyList =
    Decode.list <|
        Decode.map2 (,)
            (Decode.index 0 Decode.string)
            (Decode.index 1 Decode.int)


insertResearch : Research -> Researches -> Researches
insertResearch research dict =
    Dict.insert research.id research dict


researchLevelDecoder : Decode.Decoder ResearchLevel
researchLevelDecoder =
    Decode.succeed ResearchLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "resources" <|
                Decode.list <|
                    Decode.map2 (,)
                        (Decode.field "type" Decode.string)
                        (Decode.field "amount" Decode.int)
           )


fetchResearches : Model -> Cmd Messages.Msg
fetchResearches model =
    case model.user of
        LoggedIn user ->
            authenticatedGet model "/api/researches" researchesDecoder Messages.ReceiveResearches

        _ ->
            Cmd.none


startResearching : Model.Model -> String -> Cmd Messages.Msg
startResearching model key =
    authenticatedPost model (podUrl model.user "/researches") Decode.value Messages.QueueEntryAdded Json.Encode.null
