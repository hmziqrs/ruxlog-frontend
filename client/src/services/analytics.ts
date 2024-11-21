'use client';

import { getAnalytics } from 'firebase/analytics';
import { firebase } from './firebase';

export const analytics = getAnalytics(firebase);
