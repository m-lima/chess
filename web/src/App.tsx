import React from 'react'
import {
  BrowserRouter as Router,
  Route,
  Switch 
} from 'react-router-dom'

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
        render={() => <Board />}
      />
    </Switch>
  </Router>

export default App
