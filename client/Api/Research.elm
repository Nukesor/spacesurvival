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


researchesDecoder : Decode.Decoder (Dict.Dict String Research)
researchesDecoder =
    Decode.dict <|
        Decode.succeed Research
            |: (Decode.field "display_name" Decode.string)
            |: (Decode.field "current_level" Decode.int)
            |: (Decode.field "dependencies" <|
                    Decode.list <|
                        Decode.map2 (,)
                            (Decode.index 0 Decode.string)
                            (Decode.index 1 Decode.int)
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


fetchResearches : Model -> Cmd Messages.Msg
fetchResearches model =
    case model.user of
        LoggedIn user ->
            authenticatedGet model ("/api/pod/" ++ user.podId ++ "/researches") researchesDecoder Messages.ReceiveResearches

        _ ->
            Cmd.none


startResearching : Model.Model -> String -> Cmd Messages.Msg
startResearching model key =
    authenticatedPost model ("/api/researches/pod/" ++ key) Decode.value Messages.QueueEntryAdded Json.Encode.null
