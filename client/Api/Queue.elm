module Api.Queue exposing (..)

import Api.Util exposing (authenticatedGet, dateDecoder)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages
import Model exposing (Model)
import Model.Queue exposing (Entry(ResearchEntry), Queue, ResearchData)


queueDecoder : Decode.Decoder Queue
queueDecoder =
    Decode.list <|
        Decode.oneOf [ researchDecoder ]


queueEntryDecoder : Decode.Decoder Entry
queueEntryDecoder =
    Decode.oneOf [ researchDecoder ]


researchDecoder : Decode.Decoder Model.Queue.Entry
researchDecoder =
    (Decode.succeed ResearchData
        |: (Decode.field "created_at" dateDecoder)
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "research_name" Decode.string)
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "finishes_at" (Decode.maybe dateDecoder))
    )
        |> Decode.map ResearchEntry


fetchQueue : Model -> Cmd Messages.Msg
fetchQueue model =
    authenticatedGet model "/api/queue/pod" queueDecoder Messages.ReceiveQueue
