module Api.Queue exposing (..)

import Api.Util exposing (authenticatedGet, dataDecoder, dateDecoder)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages
import Model exposing (Model)
import Model.Queue exposing (Entry(ResearchEntry), Queue, ResearchData)


queueDecoder : Decode.Decoder Queue
queueDecoder =
    dataDecoder <|
        Decode.list <|
            Decode.oneOf [ researchDecoder ]


queueEntryDecoder =
    dataDecoder <|
        Decode.oneOf [ researchDecoder ]


researchDecoder : Decode.Decoder Model.Queue.Entry
researchDecoder =
    (Decode.succeed ResearchData
        |: (Decode.field "created_at" dateDecoder)
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "research_name" Decode.string)
        |: (Decode.field "level" Decode.int)
    )
        |> Decode.map ResearchEntry


getQueue : Model -> Cmd Messages.Msg
getQueue model =
    authenticatedGet model "/api/queue/pod" queueDecoder Messages.ReceiveQueue
