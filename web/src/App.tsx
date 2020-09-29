import React from 'react'
import {
  BrowserRouter as Router,
  Route,
  Switch 
} from 'react-router-dom'

import * as game from 'game'

import Board from './Board'
import Landing from './Landing'

const App = () =>
  <Router>
    <Switch>
      <Route
        path='/'
        exact
        render={() => <Landing />}
      />
      <Route
        path={'/game/:id'}
        exact
        render={() =>
          <Board
            color={game.Color.White}
            board={new game.Board('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1')}
          />
        }
      />
    </Switch>
  </Router>

export default App
