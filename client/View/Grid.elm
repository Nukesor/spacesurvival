module View.Grid exposing (..)

import Svg exposing (..)
import Svg.Attributes exposing (..)
import Svg.Events exposing (..)
import Model.Grid as Grid exposing (Slot, Grid)
import Model.Util exposing (..)
import Model exposing (..)
import Html exposing (..)
import ArrayExtra
import Messages exposing (..)
import Styles.BuildDialog
import Html.CssHelpers


css : Html.CssHelpers.Namespace String class id msg
css =
    Html.CssHelpers.withNamespace Styles.BuildDialog.ns


grid : Model -> Html.Html Msg
grid model =
    let
        buildDialog =
            if ArrayExtra.any (ArrayExtra.any (\slot -> slot.selectedForBuilding)) model.grid then
                div [ css.class [ Styles.BuildDialog.Container ] ]
                    [ ul
                        []
                        (List.map (\m -> li [] [ Html.text m.name ]) model.availableModules)
                    ]
            else
                div [] []
    in
        div []
            [ svg []
                (Grid.map
                    slot
                    model.grid
                )
            , buildDialog
            ]


slot : Slot -> Svg Msg
slot slot =
    let
        ( xp, yp ) =
            toPercentage slot.position
    in
        image
            [ xlinkHref "/static/img/grid_slot.svg"
            , x xp
            , y yp
            , width "10%"
            , height "10%"
            , class "slot"
            , onClick <| ShowBuildDialog slot.position
            ]
            []


toPercentage : Point -> ( String, String )
toPercentage point =
    ( (toString <| point.x * 10) ++ "%", (toString <| point.y * 10) ++ "%" )
