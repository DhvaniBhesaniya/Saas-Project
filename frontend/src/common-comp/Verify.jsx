import React from 'react';
import { useLocation } from 'react-router-dom';
import SuccessPage from './SuccessPage';  // Import SuccessPage component
import FailurePage from './FailurePage';  // Import FailurePage component

const Verify = () => {
  const location = useLocation();
  // http://localhost:5170/verify?success=true&session_id=cs_test_a1Q5f25QPXh6Bucg1rsSIVvqRcdEgjFbu0SohrTWYIs5SYlRJI4jbwuc2p
  
  // Parse query parameters
  const searchParams = new URLSearchParams(location.search);
  const success = searchParams.get('success');

  // Render SuccessPage or FailurePage based on the 'success' param
  return (
    <div>
      {success === 'true' ? <SuccessPage /> : <FailurePage />}
    </div>
  );
};

export default Verify;
