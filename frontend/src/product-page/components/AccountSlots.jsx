import * as React from 'react';
import { Account, AuthenticationContext, SessionContext } from '@toolpad/core';
import CustomMenu from './CustomMenu';


const demoSession = {
  user: {
    name: 'Bharat Kashyap',
    email: 'bharatkashyap@outlook.com',
    image: 'https://avatars.githubusercontent.com/u/19550456',
  },
};

export default function AccountSlots() {
  const [session, setSession] = React.useState(demoSession);
  const authentication = React.useMemo(() => {
    return {
      signIn: () => {
        setSession(demoSession);
      },
      signOut: () => {
        setSession(null);
      },
    };
  }, []);

  return (
    <AuthenticationContext.Provider value={authentication}>
      <SessionContext.Provider value={session}>
        <Account
          slots={{
            menuItems: CustomMenu,
          }}
        />
      </SessionContext.Provider>
    </AuthenticationContext.Provider>
  );
}
