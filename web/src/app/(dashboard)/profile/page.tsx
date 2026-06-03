'use client';

import { useState } from 'react';
import { useAuthStore } from '@/store/auth';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Separator } from '@/components/ui/separator';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { toast } from 'sonner';
import { UserProfileResponse } from '@/types';
import { Camera, Save, Lock, User } from 'lucide-react';
import {useEffect } from 'react';

export default function ProfilePage() {
  const { user, login } = useAuthStore();
  const queryClient = useQueryClient();

  // Form states
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [uploading, setUploading] = useState(false);

  // Fetch profile
  const { data: profile, isLoading } = useQuery<UserProfileResponse>({
  queryKey: ['profile'],
  queryFn: async () => {
    const response = await api.get('/api/v1/users/me');
    return response.data;
  },
});

// Set form values when profile loads
useEffect(() => {
  if (profile) {
    setUsername(profile.username);
    setEmail(profile.email);
  }
}, [profile]);

  // Update profile mutation
  const updateProfile = useMutation({
    mutationFn: async () => {
      const response = await api.put('/api/v1/users/me', { username, email });
      return response.data;
    },
    onError: (error: any) => {
      toast.error(error.response?.data || 'Failed to update profile');
    },
  });

  // Change password mutation
  const changePassword = useMutation({
    mutationFn: async () => {
      await api.put('/api/v1/users/me/password', {
        current_password: currentPassword,
        new_password: newPassword,
        confirm_password: confirmPassword,
      });
    },
    onSuccess: () => {
      toast.success('Password changed successfully');
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');
    },
    onError: (error: any) => {
      toast.error(error.response?.data || 'Failed to change password');
    },
  });

  // Upload profile picture
  const handlePictureUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setUploading(true);
    try {
      // Upload file
      const formData = new FormData();
      formData.append('file', file);
      const uploadResponse = await api.post('/api/v1/upload/profile-picture', formData, {
        headers: { 'Content-Type': 'multipart/form-data' },
      });

      // Save path to profile
      await api.put('/api/v1/users/me/profile-picture', {
        profile_picture_path: uploadResponse.data.path,
      });

      toast.success('Profile picture updated');
      queryClient.invalidateQueries({ queryKey: ['profile'] });
    } catch (error) {
      toast.error('Failed to upload picture');
    } finally {
      setUploading(false);
    }
  };

  const getInitials = (name: string) => {
    return name.split(' ').map((n) => n[0]).join('').toUpperCase().slice(0, 2);
  };

  if (isLoading) return <div>Loading...</div>;

  return (
    <div className="max-w-2xl space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Profile Settings</h1>
        <p className="text-gray-500">Manage your account information</p>
      </div>

      {/* Profile Picture */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <User className="w-5 h-5" />
            Profile Picture
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center gap-6">
            <Avatar className="h-20 w-20">
              <AvatarImage src={profile?.profile_picture_url || ''} />
              <AvatarFallback className="bg-blue-600 text-white text-xl">
                {user?.username ? getInitials(user.username) : 'U'}
              </AvatarFallback>
            </Avatar>
            <div>
              <Label
                htmlFor="picture-upload"
                className="cursor-pointer inline-flex items-center gap-2 bg-gray-100 hover:bg-gray-200 px-4 py-2 rounded-lg text-sm font-medium transition-colors"
              >
                <Camera className="w-4 h-4" />
                {uploading ? 'Uploading...' : 'Change Picture'}
              </Label>
              <input
                id="picture-upload"
                type="file"
                accept="image/*"
                className="hidden"
                onChange={handlePictureUpload}
                disabled={uploading}
              />
              <p className="text-xs text-gray-500 mt-2">
                JPG, PNG or WebP. Max 2MB.
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Personal Information */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <User className="w-5 h-5" />
            Personal Information
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="username">Username</Label>
            <Input
              id="username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="Your username"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="Your email"
            />
          </div>

          <div className="space-y-2">
            <Label>Role</Label>
            <Input value={user?.role_title || ''} disabled className="bg-gray-50" />
          </div>

          <Button
            onClick={() => updateProfile.mutate()}
            disabled={updateProfile.isPending}
            className="flex items-center gap-2"
          >
            <Save className="w-4 h-4" />
            {updateProfile.isPending ? 'Saving...' : 'Save Changes'}
          </Button>
        </CardContent>
      </Card>

      {/* Change Password */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Lock className="w-5 h-5" />
            Change Password
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="current-password">Current Password</Label>
            <Input
              id="current-password"
              type="password"
              value={currentPassword}
              onChange={(e) => setCurrentPassword(e.target.value)}
              placeholder="••••••••"
            />
          </div>

          <Separator />

          <div className="space-y-2">
            <Label htmlFor="new-password">New Password</Label>
            <Input
              id="new-password"
              type="password"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
              placeholder="••••••••"
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="confirm-password">Confirm New Password</Label>
            <Input
              id="confirm-password"
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              placeholder="••••••••"
            />
          </div>

          <Button
            onClick={() => changePassword.mutate()}
            disabled={changePassword.isPending || !currentPassword || !newPassword || !confirmPassword}
            variant="outline"
            className="flex items-center gap-2"
          >
            <Lock className="w-4 h-4" />
            {changePassword.isPending ? 'Changing...' : 'Change Password'}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}