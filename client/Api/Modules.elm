module Api.Modules exposing (..)

import Api.Util exposing (authenticatedGet, dataDecoder, pairDecoder)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages
import Model exposing (Model)
import Model.Modules exposing (Module, ModuleLevel, Shoots)
import Dict


modulesDecoder : Decode.Decoder (Dict.Dict String Module)
modulesDecoder =
    dataDecoder <|
        Decode.dict moduleDecoder


moduleDecoder : Decode.Decoder Module
moduleDecoder =
    Decode.succeed Module
        |: (Decode.field "name" Decode.string)
        |: (Decode.field "dependencies" (Decode.list (pairDecoder Decode.string Decode.int)))
        |: (Decode.field "levels" (Decode.list moduleLevelDecoder))


moduleLevelDecoder : Decode.Decoder ModuleLevel
moduleLevelDecoder =
    Decode.succeed ModuleLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "consumes" (Decode.list resourceAmountDecoder))
        |: (Decode.field "generates" (Decode.list resourceAmountDecoder))
        |: (Decode.field "resources" (Decode.list resourceAmountDecoder))
        |: (Decode.field "shoots" (Decode.maybe shootsDecoder))
        |: (Decode.field "time" Decode.int)


shootsDecoder : Decode.Decoder Shoots
shootsDecoder =
    Decode.succeed Shoots
        |: (Decode.field "damage" Decode.int)
        |: (Decode.field "range" Decode.int)
        |: (Decode.field "rate" Decode.int)


resourceAmountDecoder : Decode.Decoder ( String, Int )
resourceAmountDecoder =
    pairDecoder Decode.string Decode.int


getAvailableModules : Model -> Cmd Messages.Msg
getAvailableModules model =
    authenticatedGet model "/api/modules" modulesDecoder Messages.ReceiveAvailableModules
