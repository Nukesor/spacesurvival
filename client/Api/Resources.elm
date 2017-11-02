module Api.Resources exposing (..)

import Api.Util exposing (authenticatedGet, podUrl)
import Json.Decode as Decode
import Json.Decode.Extra exposing ((|:))
import Messages exposing (Msg(ReceiveResources))
import Model
import Model.Resources exposing (Resource)
import Model.User exposing (User, User(LoggedIn))


decodeResources : Decode.Decoder (List Resource)
decodeResources =
    Decode.list decodeResource


decodeResource : Decode.Decoder Resource
decodeResource =
    Decode.succeed Resource
        |: (Decode.field "amount" Decode.int)
        |: (Decode.field "id" Decode.string)
        |: (Decode.field "max_amount" Decode.int)
        |: (Decode.field "name" Decode.string)


fetchResources : Model.Model -> Cmd Msg
fetchResources model =
    case model.user of
        LoggedIn user ->
            authenticatedGet model (podUrl user "/resources") decodeResources ReceiveResources

        _ ->
            Cmd.none
