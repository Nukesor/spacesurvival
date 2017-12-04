module Api.Queue exposing (..)

import Api.Util exposing (authenticatedGet, createRequest, dateDecoder, podUrl)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages exposing (commandAsMsg)
import Model exposing (Model)
import Model.Queue exposing (Entry(ModuleEntry), Entry(ResearchEntry), ModuleData, Queue, ResearchData, commonData)


queueDecoder : Decode.Decoder Queue
queueDecoder =
    Decode.field "queue_entries" (Decode.list queueEntryDecoder)


queueEntryDecoder : Decode.Decoder Entry
queueEntryDecoder =
    Decode.oneOf [ researchDecoder, moduleDecoder ]


entryDecoder constructor =
    Decode.succeed constructor
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "finishes_at" (Decode.maybe dateDecoder))
        |: (Decode.field "level" Decode.int)
        |: (Decode.field "duration" Decode.int)
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "type" Decode.string)


researchDecoder : Decode.Decoder Model.Queue.Entry
researchDecoder =
    entryDecoder ResearchData
        |: Decode.field "research_id" Decode.string
        |> Decode.map ResearchEntry


moduleDecoder : Decode.Decoder Entry
moduleDecoder =
    entryDecoder ModuleData
        |: Decode.field "module_id" Decode.string
        |> Decode.map ModuleEntry


fetchQueue : Model -> Cmd Messages.Msg
fetchQueue model =
    authenticatedGet model (podUrl model.user "/queue") queueDecoder Messages.ReceiveQueue


cancelEntry : Model -> Model.Queue.Entry -> Cmd Messages.Msg
cancelEntry model entry =
    let
        uuid =
            (commonData entry).uuid
    in
        createRequest
            model.user
            "DELETE"
            (podUrl model.user "/queue/entry/" ++ uuid)
            (Decode.value)
            Nothing
            (commandAsMsg (fetchQueue model))
