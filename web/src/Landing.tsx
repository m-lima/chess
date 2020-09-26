import React from 'react'
import { Link } from 'react-router-dom'

import logo from './img/logo.svg'

import './Landing.css'

const Logo = (props: { text: string, img?: string }) =>
  <div className='Landing-Logo'>
    {props.img ? <img src={props.img} alt='' /> : <></>}
    {props.text}
  </div>

const Landing = () =>
  <div className='Landing'>
    <Logo text='Chess!' img={logo} />
    <div className='CenterContainer'>
      <div className='Button'>
        <Link to='/game/123'>
          New game
        </Link>
      </div>
      <div className='Button'>
        <Link to='/game/123'>
          Join game
        </Link>
      </div>
    </div>
  </div>

export default Landing;
