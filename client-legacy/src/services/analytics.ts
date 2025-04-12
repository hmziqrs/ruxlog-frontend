'use client';

import { getAnalytics } from 'firebase/analytics';
import { firebase } from './firebase';
import { isServer } from '@/utils';

export const analytics = !isServer() ? getAnalytics(firebase!) : null;
