module Update.User exposing (login)

import Api.Auth
import Api.Research
import Api.Queue
import Api.Modules
import Api.Resources
import Api.Modules
import Messages exposing (Msg)
import Model exposing (Model)
import Model.User exposing (LoggedInData)


login : Model -> LoggedInData -> ( Model, Cmd Msg )
login model user =
    let
        updatedModel =
            { model | user = Model.User.LoggedIn user }
    in
        updatedModel
            ! [ Api.Auth.saveToken user
              , Api.Research.fetchResearches updatedModel
              , Api.Queue.fetchQueue updatedModel
              , Api.Modules.fetchAvailableModules updatedModel
              , Api.Resources.fetchResources updatedModel
              , Api.Modules.fetchGridModules updatedModel
              ]
