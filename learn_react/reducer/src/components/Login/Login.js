import React, { useState, useEffect, useReducer } from 'react';

import Card from '../UI/Card/Card';
import classes from './Login.module.css';
import Button from '../UI/Button/Button';

const passwordReducer = (state, action) => {
  switch ( action.type ) {
    case "USER_INPUT":
      return {
        ...state,
        value: action.payload,
        isValid: state.value.trim().length > 6,
      }

    case "INPUT_BLUR":
      return {
        ...state,
        isValid: state.value.trim().length > 6,
      }

    default:
      return {
        ...state
      }
  }
}

const emailReducer = (state, action) => {
  switch ( action.type ) {
    case "USER_INPUT":
      return {
        ...state,
        value: action.payload,
      }

    case "INPUT_BLUR":
      return {
        ...state,
        isValid: state.value.includes("@"),
      }

    default:
      return {
        ...state
      }
  }

}

const Login = (props) => {
  const [formIsValid, setFormIsValid] = useState(false);

  const [ emailState, dispatchEmail ] = useReducer(emailReducer, { value: "", isValid: false });
  const [ passwordState, dispatchPassword ] = useReducer(passwordReducer, { value: "", isValid: false });

  useEffect(() => {
    const identifier = setTimeout(() => {
      setFormIsValid(emailState.isValid && passwordState.isValid);
    }, 500);

    return () => {
      clearTimeout(identifier);
    };
  }, [emailState.isValid, passwordState.isValid]);

  const emailChangeHandler = (event) => {
    dispatchEmail({type: "USER_INPUT", payload: event.target.value});
  };

  const passwordChangeHandler = (event) => {
    dispatchPassword({ type: "USER_INPUT", payload: event.target.value });
  };

  const validateEmailHandler = () => {
    dispatchEmail({type: "INPUT_BLUR"});
  };

  const validatePasswordHandler = () => {
    dispatchPassword({type: "INPUT_BLUR"});
  };

  const submitHandler = (event) => {
    event.preventDefault();
    props.onLogin(emailState.value, passwordState.value);
  };

  return (
    <Card className={classes.login}>
      <form onSubmit={submitHandler}>
        <div
          className={`${classes.control} ${
            emailState.isValid === false ? classes.invalid : ''
          }`}
        >
          <label htmlFor="email">E-Mail</label>
          <input
            type="email"
            id="email"
            value={emailState.value}
            onChange={emailChangeHandler}
            onBlur={validateEmailHandler}
          />
        </div>
        <div
          className={`${classes.control} ${
            passwordState.isValid === false ? classes.invalid : ''
          }`}
        >
          <label htmlFor="password">Password</label>
          <input
            type="password"
            id="password"
            value={passwordState.value}
            onChange={passwordChangeHandler}
            onBlur={validatePasswordHandler}
          />
        </div>
        <div className={classes.actions}>
          <Button type="submit" className={classes.btn} disabled={!formIsValid}>
            Login
          </Button>
        </div>
      </form>
    </Card>
  );
};

export default Login;
