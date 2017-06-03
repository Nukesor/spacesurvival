module Api.Modules exposing (..)

import Api.Util exposing (authenticatedGet, dataDecoder, pairDecoder)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages
import Model exposing (Model)
import Model.Modules exposing (Module, ModuleLevel, Shoots)


modulesDecoder =
    dataDecoder <|
        Decode.dict moduleDecoder


moduleDecoder =
    Decode.succeed Module
        |: (Decode.field "name" Decode.string)
        |: (Decode.field "dependencies" (Decode.list (pairDecoder Decode.string Decode.int)))
        |: (Decode.field "levels" (Decode.list moduleLevelDecoder))


moduleLevelDecoder =
    Decode.succeed ModuleLevel
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "consumes" (Decode.list resourceAmountDecoder))
        |: (Decode.field "generates" (Decode.list resourceAmountDecoder))
        |: (Decode.field "resources" (Decode.list resourceAmountDecoder))
        |: (Decode.field "shoots" (Decode.maybe shootsDecoder))
        |: (Decode.field "time" Decode.int)


shootsDecoder =
    Decode.succeed Shoots
        |: (Decode.field "damage" Decode.int)
        |: (Decode.field "range" Decode.int)
        |: (Decode.field "rate" Decode.int)


resourceAmountDecoder =
    pairDecoder Decode.string Decode.int


getAvailableModules model =
    authenticatedGet model "/api/modules" modulesDecoder Messages.ReceiveAvailableModules
