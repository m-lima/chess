import React, { useState, useEffect, useCallback } from 'react'

import * as game from 'game'

import './Board.css'

import blackPawn from './img/black-pawn.svg'
import blackKnight from './img/black-knight.svg'
import blackRook from './img/black-rook.svg'
import blackBishop from './img/black-bishop.svg'
import blackQueen from './img/black-queen.svg'
import blackKing from './img/black-king.svg'

import whitePawn from './img/white-pawn.svg'
import whiteKnight from './img/white-knight.svg'
import whiteRook from './img/white-rook.svg'
import whiteBishop from './img/white-bishop.svg'
import whiteQueen from './img/white-queen.svg'
import whiteKing from './img/white-king.svg'

const toBackgroundId = (index: number) => {
  return Math.floor(index / 8) % 2 === 0
    ? index % 2 === 0 ? 'white' : 'black'
    : index % 2 !== 0 ? 'white' : 'black'
}

const renderToken = (fen_char: number) => {
  switch (fen_char) {
    default: return <></>
    case 112: return <img src={blackPawn} alt='p' />
    case 110: return <img src={blackKnight} alt='n' />
    case 114: return <img src={blackRook} alt='r' />
    case  98: return <img src={blackBishop} alt='b' />
    case 113: return <img src={blackQueen} alt='q' />
    case 107: return <img src={blackKing} alt='k' />
    case  80: return <img src={whitePawn} alt='P' />
    case  78: return <img src={whiteKnight} alt='N' />
    case  82: return <img src={whiteRook} alt='R' />
    case  66: return <img src={whiteBishop} alt='B' />
    case  81: return <img src={whiteQueen} alt='Q' />
    case  75: return <img src={whiteKing} alt='K' />
  }
}

const Board = () => {
  const color = game.Color.White;
  const [bla, setBla] = useState(new game.Board())
  const [board, setBoard] = useState('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1')
  const [codes, setCodes] = useState<number[]>([])
  const [selected, setSelected] = useState<number>()
  const [attack, setAttack] = useState<number>()
  const [highlighted, setHighlighted] = useState<number[]>([])

  useEffect(() => {
    const clean = board.substring(0, board.indexOf(' '))
    setCodes([...clean].flatMap(c => {
      const code = c.charCodeAt(0)
      if (code === 47) {
        return []
      } else if (code > 48 && code < 57) {
        return Array.from(Array(code - 48))
      } else {
        return [code]
      }
    }))
  }, [board])

  const select = (index: number) => {
    setHighlighted(bla.legal_moves(index))
    setSelected(index)
  }

  const render = (code: number, index: number) =>
    <div
      className='CellBackground'
      key={index}
      onClick={() => select(index)}
      id={toBackgroundId(index)}
    >
      <div
        className='Cell'
        onClick{() => {
          if (highlighted.indexOf(index) < 0) return

          bla.move(selected, index)
        }}
        id={index === selected ? 'selected' : highlighted.findIndex(i => i === index) >= 0 ? 'highlighted' : ''}
      >
        {renderToken(code)}
      </div>
    </div>

  return (
    <>
      <div className='Board'>
        {codes.map(render)}
      </div>
      <div>1</div>
      <div>2</div>
      <div>3</div>
      <div>4</div>
      <div>5</div>
      <div>6</div>
      <div>7</div>
      <div>8</div>
      <div>a b c d e f g h</div>
    </>
  )
}

export default Board;

