module Update exposing (..)

import Animation
import Api.Auth
import Api.Modules exposing (fetchGridModules, startBuilding, upgrade)
import Api.Queue exposing (fetchQueue)
import Api.Research exposing (fetchResearches, startResearching)
import Messages exposing (..)
import Model exposing (..)
import Model.Grid exposing (modules)
import Model.Modules
import Model.Queue exposing (unfinishedEntries)
import Model.User exposing (LoginData)
import Result exposing (withDefault)
import Time.DateTime
import Update.User


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        AnimateModal frame ->
            { model | authDialogAnimation = Animation.update frame model.authDialogAnimation } ! []

        Messages.Login ->
            model ! [ Api.Auth.login model ]

        Messages.Register ->
            model ! [ Api.Auth.register model ]

        UpdateUser user ->
            { model | user = user } ! []

        Registered result ->
            case Debug.log "register result" result of
                Ok user ->
                    { model | user = Model.User.LoggingIn user } ! []

                Err _ ->
                    model ! []

        Messages.LoggedIn result ->
            case Debug.log "login result" result of
                Ok user ->
                    Update.User.login model user

                Err err ->
                    model ! []

        ShowBuildDialog maybePoint ->
            { model
                | buildingAt = maybePoint
            }
                ! []

        ReceiveResearches result ->
            case Debug.log "researches" result of
                Ok researches ->
                    { model | researches = researches } ! []

                Err err ->
                    logout model ! []

        ReceiveQueue result ->
            case Debug.log "queue" result of
                Ok queue ->
                    let
                        commands =
                            if queue /= model.queue then
                                [ fetchResearches model ]
                            else
                                []
                    in
                        { model | queue = queue } ! commands

                Err err ->
                    logout model ! []

        SetMainView view ->
            { model | mainView = view } ! []

        QueueEntryAdded result ->
            case Debug.log "queue entry" result of
                Ok _ ->
                    model ! [ fetchQueue model ]

                Err err ->
                    model ! []

        StartResearching key ->
            model ! [ startResearching model key ]

        ReceiveAvailableModules result ->
            case Debug.log "available modules" result of
                Ok modules ->
                    { model | availableModules = modules } ! []

                Err err ->
                    model ! []

        ReceiveResources result ->
            result
                |> Result.map (\resources -> { model | resources = resources } ! [])
                |> withDefault (model ! [])

        Tick time ->
            let
                dt =
                    Maybe.map (\lastTick -> time - lastTick) model.lastTick
            in
                case dt of
                    Just dt ->
                        let
                            updatedQueue =
                                unfinishedEntries model.currentDate model.queue

                            updatedResources =
                                Model.Modules.tick dt (modules model.pod) model.availableModules model.resources

                            commands =
                                if model.queue /= updatedQueue then
                                    [ fetchQueue model, fetchGridModules model ]
                                else
                                    []
                        in
                            { model
                                | currentDate = Time.DateTime.fromTimestamp time
                                , resources = updatedResources
                                , lastTick = Just time
                            }
                                ! commands

                    Nothing ->
                        { model | lastTick = Just time } ! []

        ReceiveGrid result ->
            result
                |> Result.map (\modules -> { model | pod = modules } ! [])
                |> withDefault (model ! [])

        StartBuilding id point ->
            { model | buildingAt = Nothing } ! [ startBuilding model id point ]

        Upgrade id ->
            model ! [ upgrade model id ]


logout : Model -> Model
logout model =
    { model | user = Model.User.LoggingIn { identifier = "", password = "" } }
