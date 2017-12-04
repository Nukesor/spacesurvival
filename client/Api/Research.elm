module Api.Research exposing (..)

import Api.Queue exposing (fetchQueue)
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
            |: (Decode.field "levels" (Decode.list researchLevelSpecDecoder))


dependencyList : Decode.Decoder (List ( ResourceId, Int ))
dependencyList =
    Decode.list <|
        Decode.map2 (,)
            (Decode.index 0 Decode.string)
            (Decode.index 1 Decode.int)


insertResearch : Research -> Researches -> Researches
insertResearch research dict =
    Dict.insert research.id research dict


researchLevelSpecDecoder : Decode.Decoder ResearchLevel
researchLevelSpecDecoder =
    Decode.succeed ResearchLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "resources" <|
                Decode.list <|
                    Decode.map2 (,)
                        (Decode.field "type" Decode.string)
                        (Decode.field "amount" Decode.int)
           )


currentResearchLevelDecoder : Decode.Decoder ( ResearchId, Int )
currentResearchLevelDecoder =
    Decode.map2 (,)
        (Decode.field "type" Decode.string)
        (Decode.field "level" Decode.int)


fetchAvailableResearches : Model -> Cmd Messages.Msg
fetchAvailableResearches model =
    case model.user of
        LoggedIn user ->
            authenticatedGet model "/api/researches" researchesDecoder Messages.ReceiveAvailableResearches

        _ ->
            Cmd.none


fetchResearchLevels : Model -> Cmd Messages.Msg
fetchResearchLevels model =
    authenticatedGet model (podUrl model.user "/researches") (Decode.list currentResearchLevelDecoder) Messages.ReceiveResearchLevels


startResearching : Model.Model -> String -> Cmd Messages.Msg
startResearching model key =
    let
        researchObject =
            Json.Encode.object [ ( "type", Json.Encode.string key ) ]
    in
        authenticatedPost
            model
            (podUrl model.user "/researches")
            Decode.value
            (Messages.commandAsMsg (fetchQueue model))
            researchObject
