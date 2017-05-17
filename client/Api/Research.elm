module Api.Research exposing (..)

import Json.Decode as Decode
import Model.Research exposing (..)
import Api.Util exposing (..)


researchesDecoder =
    dataDecoder <|
        Decode.dict <|
            Decode.succeed Research
                |: (Decode.field "name" Decode.string)
                |: (Decode.field "current_level" Decode.int)
                |: (Decode.field "dependencies" Decode.map2 (,) Decode.string Decode.int)
                |: (Decode.field "levels" researchLevelDecoder)


researchLevelDecoder =
    Decode.succeed ResearchLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "resources" Decode.map2 (,) Decode.string Decode.int)
