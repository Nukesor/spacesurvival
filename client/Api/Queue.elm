module Api.Queue exposing (..)

import Api.Util exposing (authenticatedGet, dateDecoder, podUrl)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages
import Model exposing (Model)
import Model.Queue exposing (Entry(ModuleEntry), Entry(ResearchEntry), ModuleData, Queue, ResearchData)


queueDecoder : Decode.Decoder Queue
queueDecoder =
    Decode.field "queue_entries" (Decode.list queueEntryDecoder)


queueEntryDecoder : Decode.Decoder Entry
queueEntryDecoder =
    Decode.oneOf [ researchDecoder, moduleDecoder ]


entryDecoder constructor =
    Decode.succeed constructor
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "started_at" (Decode.maybe dateDecoder))
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "duration" Decode.int)


researchDecoder : Decode.Decoder Model.Queue.Entry
researchDecoder =
    (entryDecoder ResearchData
        |: (Decode.field "research" Decode.string)
    )
        |> Decode.map ResearchEntry


moduleDecoder : Decode.Decoder Entry
moduleDecoder =
    (entryDecoder ModuleData
        |: (Decode.field "module" Decode.string)
    )
        |> Decode.map ModuleEntry


fetchQueue : Model -> Cmd Messages.Msg
fetchQueue model =
    authenticatedGet model (podUrl model.user "/queue") queueDecoder Messages.ReceiveQueue
