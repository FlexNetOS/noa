export interface UpdateUserData {
  firstName?: string;
  lastName?: string;
  email?: string;
  mobileNumber?: string;
  bio?: string;
  timezone?: string;
  language?: string;
  status?: string;
  emailVerified?: boolean;
  avatar?: string;
  preferences?: {
    theme?: string;
    notifications?: {
      email?: boolean;
      push?: boolean;
    };
  };
  onboardInfo?: {
    [key: string]: string;
  };
}

export interface UpdateEmailData {
  email: string;
}
