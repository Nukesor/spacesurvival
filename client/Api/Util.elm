module Api.Util exposing (..)

import Json.Decode as Decode


dataDecoder : Decode.Decoder a -> Decode.Decoder a
dataDecoder =
    Decode.field "data"
