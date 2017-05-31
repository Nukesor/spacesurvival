module Api.Queue exposing (..)

import Api.Util exposing (authenticatedGet, dataDecoder)
import Json.Decode
import Messages
import Model exposing (Model)


queueDecoder : Json.Decode.Decoder Json.Decode.Value
queueDecoder =
    dataDecoder <|
        Json.Decode.value


getQueue : Model -> Cmd Messages.Msg
getQueue model =
    authenticatedGet model "/api/queue/pod" queueDecoder Messages.ReceiveQueue
